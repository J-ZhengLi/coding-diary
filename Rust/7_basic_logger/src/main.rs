use logger_core::Logger;

fn main() {
    Logger::new().colored().init().unwrap();

    Logger::err("THIS IS AN ERROR MESSAGE!");
    Logger::warn("this is a warning.");
    Logger::debug("this is a debug message.");
    Logger::info("this is just a piece of infomation.");
    Logger::trace("this is a trace message");
}
