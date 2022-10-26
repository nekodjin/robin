use internment::Intern;

pub type Id<T> = Intern<T>;

pub type IString = Id<str>;

pub fn intern(s: &str) -> IString {
    IString::from(s)
}
