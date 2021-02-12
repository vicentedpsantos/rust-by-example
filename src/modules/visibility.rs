// Rust provides a powerful module system that can be used to hierarchically split code in
// logical units (modules), and manager visibility (public/private) between them.

// A module is a collection of items: functions, structs, traits, `impl` blocks, and even
// other modules
//
// By default, the items in a module have private visibility, but this can be overriden
// with the `pub` modifier. Only the specific items of a module can be accessed from
// outside the module scope.

// A module named `my_mod`
mod my_mod {
    // Items in a module default to private visibility
    fn private_function() {
    	println!("Called `my_mod::private_function()`");
    }

    // Use the `pub` modifier to override default visibility
    pub fn function() {
    	println!("Called `my_mod::function()`");
    }

    // Items can access other items in the same module,
    // even when private
    pub fn indirect_access() {
    	print!("Called `my_mod::indirect_access()`, that ");
	private_function();
    }

    // Modules can be nested
    pub mod nested {
    	pub fn function() {
    	    println!("Called `my_mod::nested::function()`");
    	}

	#[allow(dead_code)]
	fn private_function() {
	    println!("Called `my_mod::nested::private_function()`");
	}

	// Functions declared using `pub(in path)` syntax are only visible
	// within a given path. `path` must be a parent or ancestor module
	pub(in crate::my_mod) fn public_function_in_my_mod() {
	    println!("Called `my_mod::nested::public_function_in_my_mod()`");
	}

	// Functions declared using `pub(self)` are only visible within the module
	// which is the same as creating them as private
	pub(self) fn public_function_in_super_mod() {
	    println!("Called `my_mod::nested::public_function_in_super_mod()`");
	}
    }

    pub fn call_public_function_in_my_mod() {
    	print!("Called `my_mod::call_public_function_in_my_mod()`, that\n> ");
	nested::public_function_in_my_mod();
	print!("> ");
	// nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
    	println!("Called `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
    	#[allow(dead_code)]
	pub fn function() {
	    println!("Called `my_mod::private_nested::function()`");
	}

	// Private parent items will still restrict the visibility of a child item,
	// even if its declared as visible within a bigger scope
	#[allow(dead_code)]
	pub(crate) fn restricted_function() {
	    println!("Called `my_mod::private_nested::restricted_function()`");
	}
    }
}

fn function() {
    println!("Called `function()`");
}

fn main() {
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_crate();
}
