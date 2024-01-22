use crate::{Descriptor, Gauge, Info, Metric};
use std::fmt::Write;

pub struct Encoder<'a> {
    writer: &'a mut dyn Write,
}

impl<'a> Encoder<'a> {
    pub fn new(writer: &'a mut dyn Write) -> Self {
        Self { writer }
    }

    pub fn encode_descriptor(&mut self, descriptor: &Descriptor) -> Result<(), std::fmt::Error> {
        let name = descriptor.name();

        self.writer.write_str("# HELP ")?;
        self.writer.write_str(&name)?;
        self.writer.write_str(" ")?;
        self.writer.write_str(descriptor.description())?;
        self.writer.write_str("\n")?;

        if let Some(unit) = descriptor.unit() {
            self.writer.write_str("# UNIT ")?;
            self.writer.write_str(&name)?;
            self.writer.write_str(" ")?;
            self.writer.write_str(unit)?;
            self.writer.write_str("\n")?;
        }

        Ok(())
    }

    pub fn encode_gauge(&mut self, gauge: &Gauge) -> Result<(), std::fmt::Error> {
        self.encode_metric(gauge.metric())?;
        self.writer.write_fmt(format_args!("{}", gauge.value()))?;

        if let Some(timestamp) = gauge.timestamp() {
            self.writer.write_fmt(format_args!(" {}", timestamp))?;
        }

        self.writer.write_str("\n")
    }

    pub fn encode_info(&mut self, info: &Info) -> Result<(), std::fmt::Error> {
        self.encode_metric(info.metric())?;
        self.writer
            .write_fmt(format_args!("{}", info.timestamp()))?;
        self.writer.write_str("\n")
    }

    pub fn encode_metric(&mut self, metric: &Metric) -> Result<(), std::fmt::Error> {
        self.writer.write_str(metric.name())?;
        self.writer.write_str("{")?;
        self.writer.write_str(metric.labels())?;
        self.writer.write_str("} ")
    }
}
