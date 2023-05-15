#![allow(unused)]
use std::rc::Rc;
use std::fmt::{Display, Formatter, Result};

struct Owner(String);

impl Display for Owner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Owner({})", self.0)
    }
}

struct Tool {
    id: i32,
    owner: Rc<Owner>,
}

impl Display for Tool {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Tool({}): {}", self.id, self.owner)
    }
}

fn main() {
    // Create a reference-counted `Owner`.
    let tool_owner: Rc<Owner> = Rc::new(
        Owner(format!("Bob Dobbs")),
    );

    // Create `Tool`s belonging to `tool_owner`. Cloning the `Rc<Owner>`
    // gives us a new pointer to the same `Owner` allocation, incrementing
    // the reference count in the process.
    let tool1 = Tool {
        id: 1,
        owner: Rc::clone(&tool_owner),
    };
    let tool2 = Tool {
        id: 2,
        owner: Rc::clone(&tool_owner),
    };

    // Dispose of our local variable `tool_owner`.
    drop(tool_owner);

    // Despite dropping `tool_owner`, we're still able to print out the name
    // of the `Owner` of the `Tool`s. This is because we've only dropped a
    // single `Rc<Owner>`, not the `Owner` it points to. As long as there are
    // other `Rc<Owner>` pointing at the same `Owner` allocation, it will remain
    // live. The field projection `tool1.owner.name` works because
    // `Rc<Owner>` automatically dereferences to `Owner`.
    println!("{tool1}");
    println!("{tool2}");

    // At the end of the function, `tool1` and `tool2` are destroyed, and
    // with them the last counted references to our `Owner`. The Owner now
    // gets destroyed as well.
}
