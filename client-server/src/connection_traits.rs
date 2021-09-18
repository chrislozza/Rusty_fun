


trait Client {
    fn new(settings_file: &'static str) -> Self;

    fn connect(&self);
    fn disconnect(&self);
    fn is_alive(&self) -> bool;
}

trait Server {
    fn new(settings_file: &'static str) -> Self;

    fn listen(&self);
    fn disconnect(&self);
    fn is_alive(&self) -> bool;
}
