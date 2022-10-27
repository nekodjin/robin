use bigint::ubig;
use intern::IString;
use intern::Id;

pub struct Module {
    pub members: Vec<Member>,
}

pub struct Member {
    pub modifier: Accessibility,
    pub definition: MemberDefinition,
}

pub enum Accessibility {
    Public,
    Private,
}

pub enum Mutability {
    Immutable,
    Mutable,
}

pub enum MemberDefinition {
    Function(Function),
    Module(Module),
}

pub struct Function {
    pub arguments: Vec<Argument>,
    pub return_type: Type,
    pub body: FunctionBody,
}

pub struct Argument {
    pub type_annot: Type,
    pub identifier: IString,
}

pub struct FunctionBody {
    pub statements: Vec<Statement>,
}

pub struct Type {
    pub ident: IString,
}

pub enum Statement {
    LetBinding(LetBinding),
    ExpressionStatement(Expression),
}

pub struct LetBinding {
    pub ident: IString,
    pub type_annot: Type,
    pub mutability: Mutability,
    pub value: Option<Expression>,
}

pub enum Expression {
    Return(Box<Self>),
    IntLiteral(Id<ubig>),
    StringLiteral(IString),
    AddressOf(Box<Self>),
    Dereference(Box<Self>),
}
