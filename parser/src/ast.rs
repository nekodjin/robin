use intern::IString;

/// Represents a Robin module.
pub struct Module {
    /// The members of the module.
    pub members: Vec<Member>,
}

/// A member of a module.
pub struct Member {
    /// The accessibility modifier of that member.
    pub modifier: Modifier,
    /// The definition of that member.
    pub definition: MemberDefinition,
}

/// An accessibility modifier.
pub enum Modifier {
    /// Visible to the superior module.
    Public,
    /// Visible only to inferior modules.
    Private,
}

/// The definition of a module member.
pub enum MemberDefinition {
    Function(Function),
    Module(Module),
}

/// A function definition
pub struct Function {
    /// The arguments of the function
    pub arguments: Vec<Argument>,
    /// TODO
    pub return_type: (),
    /// The body of the function
    pub body: FunctionBody,
}

pub struct Argument {
    /// TODO
    pub type_annot: (),
    pub identifier: IString,
}

pub struct FunctionBody {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    // TODO
}
