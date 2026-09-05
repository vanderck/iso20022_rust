use libxml::tree::node::Node;

// ── XMLDSig algorithm URIs ─────────────────────────────────────────────────
pub(crate) const XMLDSIG_NS: &str = "http://www.w3.org/2000/09/xmldsig#";
pub(crate) const ALG_C14N_EXCL: &str = "http://www.w3.org/2001/10/xml-exc-c14n#";
pub(crate) const ALG_RSA_SHA256: &str = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256";
pub(crate) const ALG_SHA256: &str = "http://www.w3.org/2001/04/xmlenc#sha256";
pub(crate) const ALG_ENVELOPED_SIG: &str =
    "http://www.w3.org/2000/09/xmldsig#enveloped-signature";

/// Maximum XML tree depth searched by [`find_node_recursive`].
///
/// Prevents stack overflow on pathologically or maliciously deep XML documents
/// (e.g. a 100 000-level nested payload sent to the webhook / verifier).
/// ISO 20022 business messages are typically 4–10 levels deep; 64 is generous.
const MAX_XML_SEARCH_DEPTH: usize = 64;

/// Depth-first search for the first descendant whose *local* name equals `name`.
///
/// Namespace prefixes are ignored — this matches `<ds:SignedInfo>` and
/// `<SignedInfo>` alike.
///
/// Search is bounded to [`MAX_XML_SEARCH_DEPTH`] levels.  Returns `None` if
/// the node is not found within that limit, preventing unbounded recursion on
/// crafted inputs.
pub(crate) fn find_node_recursive(parent: &Node, name: &str) -> Option<Node> {
    find_at_depth(parent, name, 0)
}

fn find_at_depth(parent: &Node, name: &str, depth: usize) -> Option<Node> {
    if depth >= MAX_XML_SEARCH_DEPTH {
        return None;
    }
    let mut child = parent.get_first_child();
    while let Some(c) = child {
        if c.get_name() == name {
            return Some(c);
        }
        if let Some(found) = find_at_depth(&c, name, depth + 1) {
            return Some(found);
        }
        child = c.get_next_sibling();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use libxml::parser::Parser;

    fn parse(xml: &str) -> libxml::tree::Document {
        Parser::default().parse_string(xml).expect("test fixture XML should parse")
    }

    #[test]
    fn finds_a_shallow_child() {
        let doc = parse("<Root><Foo>hi</Foo></Root>");
        let root = doc.get_root_element().unwrap();
        assert!(find_node_recursive(&root, "Foo").is_some());
    }

    #[test]
    fn finds_a_deeply_nested_descendant() {
        let doc = parse("<Root><A><B><C><Target>x</Target></C></B></A></Root>");
        let root = doc.get_root_element().unwrap();
        let found = find_node_recursive(&root, "Target");
        assert!(found.is_some());
        assert_eq!(found.unwrap().get_content(), "x");
    }

    #[test]
    fn returns_none_when_absent() {
        let doc = parse("<Root><A><B/></A></Root>");
        let root = doc.get_root_element().unwrap();
        assert!(find_node_recursive(&root, "NotThere").is_none());
    }

    #[test]
    fn ignores_namespace_prefixes_matching_local_name() {
        let doc = parse(
            r#"<Root xmlns:ds="http://www.w3.org/2000/09/xmldsig#"><ds:SignedInfo>x</ds:SignedInfo></Root>"#,
        );
        let root = doc.get_root_element().unwrap();
        let found = find_node_recursive(&root, "SignedInfo");
        assert!(found.is_some(), "should match <ds:SignedInfo> by local name alone");
    }

    #[test]
    fn returns_first_match_in_document_order() {
        let doc = parse("<Root><X><Match>first</Match></X><Match>second</Match></Root>");
        let root = doc.get_root_element().unwrap();
        let found = find_node_recursive(&root, "Match").unwrap();
        assert_eq!(found.get_content(), "first");
    }

    #[test]
    fn search_is_bounded_and_does_not_find_a_node_past_max_depth() {
        // Build a chain of MAX_XML_SEARCH_DEPTH + 5 nested elements with the
        // target only at the very bottom - the bounded search must return
        // None rather than recursing arbitrarily deep (this is the guard
        // against a maliciously/pathologically deep XML payload).
        let depth = MAX_XML_SEARCH_DEPTH + 5;
        let mut xml = String::from("<Root>");
        for _ in 0..depth {
            xml.push_str("<Wrap>");
        }
        xml.push_str("<Target>x</Target>");
        for _ in 0..depth {
            xml.push_str("</Wrap>");
        }
        xml.push_str("</Root>");

        let doc = parse(&xml);
        let root = doc.get_root_element().unwrap();
        assert!(
            find_node_recursive(&root, "Target").is_none(),
            "search should not find a node beyond MAX_XML_SEARCH_DEPTH"
        );
    }

    #[test]
    fn search_still_finds_a_node_just_within_max_depth() {
        let depth = MAX_XML_SEARCH_DEPTH - 2;
        let mut xml = String::from("<Root>");
        for _ in 0..depth {
            xml.push_str("<Wrap>");
        }
        xml.push_str("<Target>x</Target>");
        for _ in 0..depth {
            xml.push_str("</Wrap>");
        }
        xml.push_str("</Root>");

        let doc = parse(&xml);
        let root = doc.get_root_element().unwrap();
        assert!(find_node_recursive(&root, "Target").is_some());
    }
}
