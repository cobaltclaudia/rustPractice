mod print;
mod vars;
mod string;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointers_ref;
mod structs;

fn main() {
    print::print_funct();
    vars::vars_funct();
    string::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointers_ref::run();
    structs::run();
}
