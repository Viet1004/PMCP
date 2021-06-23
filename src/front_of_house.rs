pub mod hosting {
    pub fn add_to_waitlist() {
        println!("Hello, you are in the waiting list. Thank you for your patience");
    }
    pub fn threetime_print(){
        add_to_waitlist();
        add_to_waitlist();
        add_to_waitlist();
    }
}


