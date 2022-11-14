use rodio::{Sink, OutputStream, OutputStreamHandle};

pub(crate) struct Speaker{
    stream: OutputStream,
    streamHandle: OutputStreamHandle,
    sink: Option<Sink>
}

impl Speaker{
    pub fn new() -> Speaker{
        let (stream, streamHandle) = OutputStream::try_default().unwrap();
        Speaker {
            stream,
            streamHandle,
            sink: None
        }
    }

    pub fn play(&mut self, frequency: Option<f32>){
        if self.sink.is_none() {
            let freq = frequency.unwrap_or(440.0);
            let source = rodio::source::SineWave::new(freq);
            let sink = Sink::try_new(&self.streamHandle).unwrap();
            sink.append(source);
            self.sink = Some(sink);
        }
    }

    pub fn stop(&mut self){
        if !self.sink.is_none() {
            self.sink.as_ref().unwrap().stop();
            self.sink = None;
        }
    }
}