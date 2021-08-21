use crate::node::Node;

enum InsertionMode {
    Initial,
    BeforeHTML,
    BeforeHead,
    InHead,
    InHeadNoScript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
    InCaption,
    InColumnGroup,
    InTableBody,
    InRow,
    InCell,
    InSelect,
    InSelectTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,
}

struct Parser {
    insetrion_mode: InsertionMode,
    stack_of_open_elements: Vec<Node>,
}
