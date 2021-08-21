pub struct Node {
    ty: NodeType,
    name: NodeName,
}

pub enum NodeType {
    ElementNode,
    TextNode,
    CDataSectionNode,
    ProcessingInstructionNode,
    CommentNode,
    DocumentNode,
    DocumentTypeNode,
    DocumentFragmentNode,
}

pub enum NodeName {
    Element,
    Attr,
    Text,
    CDataSection,
    ProcessingInstruction,
    Comment,
    Document,
    DocumentType,
    DocumentFragment,
}
