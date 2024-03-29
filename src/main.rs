use anyhow::Error;
use scribblekit::app::App;
use winit::error::EventLoopError;

fn main() {
    println!("Hello, world!");
    let app = App::new("ScribbleKit", 720, 480);

    let run_result = app.run();
    if let Result::Err(err) = run_result {
        match err {
            EventLoopError::NotSupported(_) => todo!(),
            EventLoopError::Os(_) => todo!(),
            EventLoopError::AlreadyRunning => todo!(),
            EventLoopError::RecreationAttempt => todo!(),
            EventLoopError::ExitFailure(_) => todo!(),
        }
    }
}
