
mod GenericComm {
    /// interface definition for communication
    pub trait CommInter {
        fn speak(&self);
        fn read(&self); 
    }
}
    
mod Windows {
    use crate::GenericComm::CommInter;

    pub struct WindowsSocket {
        
    }

    impl CommInter for WindowsSocket {
        fn speak(&self) {
            todo!()
        }

        fn read(&self) {
            todo!()
        }
    }
}

mod Linux {
    use crate::GenericComm::CommInter;

    pub struct LinuxSocket {
        
    }

    impl CommInter for LinuxSocket {
        fn speak(&self) {
            todo!()
        }
        fn read(&self) {
            todo!()
        }
    }
}

mod EmailClient {
    use crate::GenericComm::CommInter;

    pub struct EmailClient<T> {
        comm_handle: T
    }

    impl<T> EmailClient<T>
    where
        T: CommInter
    {
        pub fn new(comm_handle: T) -> Self {
            Self {
                comm_handle
            }
        }

        pub fn do_work(&self) {
            self.comm_handle.speak();
            self.comm_handle.read();
            // do stuff.
        }
    }
}


// Some application crate that wishes to use the email client and
// chooses to use the windows communication strat. 
mod UserApp {
    use crate::{GenericComm::CommInter, EmailClient::EmailClient, Windows};

    pub fn app_main() {
        let client = EmailClient::new(Windows::WindowsSocket {});
    }
}

fn main() {
    UserApp::app_main();
}
