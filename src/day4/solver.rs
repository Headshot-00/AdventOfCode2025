use crate::adv_errors::UpdateError;
use log::warn;
use ocl::{Buffer, ProQue};

const CONV_CL: &str = include_str!("conv2d_threshold.cl");

// Yeah, this whole thing would probably be faster on the CPU for this small input

pub struct Day4Solver {
    matrix: Vec<Vec<i8>>,
    kernel: Vec<f32>,
    activation: f32,
    width: usize,
    height: usize,
    pro_que: Option<ProQue>,
    buffer_a: Option<Buffer<f32>>,
    buffer_b: Option<Buffer<f32>>,
    buffer_accessible: Option<Buffer<i32>>,
    kernel_cl: Option<ocl::Kernel>,
    iteration: u8,
}
impl Day4Solver {
    pub fn new(kernel: Vec<f32>, activation: f32) -> Self {
        Day4Solver {
            matrix: Vec::new(),
            kernel: kernel,
            activation: activation,
            width: 0,
            height: 0,
            pro_que: None,
            buffer_a: None,
            buffer_b: None,
            buffer_accessible: None,
            kernel_cl: None,
            iteration: 0,
        }
    }

    pub fn init_gpu(&mut self) -> ocl::Result<()> {
        self.width = self.matrix.first().map(|r| r.len()).unwrap_or(0);
        self.height = self.matrix.len();

        let pro_que = ProQue::builder()
            .src(CONV_CL)
            .dims((self.width, self.height))
            .build()?;

        let input_len = self.width * self.height;

        let buffer_a = Buffer::<f32>::builder()
            .queue(pro_que.queue().clone())
            .flags(ocl::flags::MEM_READ_WRITE)
            .len(input_len)
            .copy_host_slice(
                &self
                    .matrix
                    .iter()
                    .flat_map(|r| r.iter().map(|&v| v as f32))
                    .collect::<Vec<_>>(),
            )
            .build()?;

        let buffer_b = Buffer::<f32>::builder()
            .queue(pro_que.queue().clone())
            .flags(ocl::flags::MEM_READ_WRITE)
            .len(input_len)
            .build()?;

        let buffer_accessible = Buffer::<i32>::builder()
            .queue(pro_que.queue().clone())
            .flags(ocl::flags::MEM_READ_WRITE)
            .len(1)
            .build()?;

        let buffer_kernel = Buffer::<f32>::builder()
            .queue(pro_que.queue().clone())
            .flags(ocl::flags::MEM_READ_ONLY | ocl::flags::MEM_COPY_HOST_PTR)
            .len(self.kernel.len())
            .copy_host_slice(&self.kernel)
            .build()?;

        let kernel_cl = pro_que
            .kernel_builder("conv2d_threshold")
            .arg_named("input", &buffer_a)
            .arg_named("output", &buffer_b)
            .arg_named("accessible_count", &buffer_accessible)
            .arg_named("k", &buffer_kernel)
            .arg(self.activation)
            .arg(self.width as i32)
            .arg(self.height as i32)
            .build()?;

        self.pro_que = Some(pro_que);
        self.buffer_a = Some(buffer_a);
        self.buffer_b = Some(buffer_b);
        self.buffer_accessible = Some(buffer_accessible);
        self.kernel_cl = Some(kernel_cl);
        self.iteration = 0;

        Ok(())
    }

    pub fn add_row(&mut self, line: &str) -> Result<(), UpdateError> {
        if line.is_empty() {
            warn!("Ignored empty line.");
            return Ok(());
        }

        let row: Vec<i8> = line
            .chars()
            .map(|c| match c {
                '@' => Ok(1),
                '.' => Ok(0),
                _ => Err(UpdateError::InvalidInput),
            })
            .collect::<Result<_, _>>()?;

        self.matrix.push(row);
        Ok(())
    }

    pub fn finalize_input(&mut self) {
        self.height = self.matrix.len();
        self.width = self.matrix.first().map(|r| r.len()).unwrap_or(0);
    }

    pub fn solve(&mut self) -> Result<u32, UpdateError> {
        let (input_buf, output_buf) = if self.iteration % 2 == 0 {
            (&self.buffer_a, &self.buffer_b)
        } else {
            (&self.buffer_b, &self.buffer_a)
        };

        self.iteration += 1;

        // Reset accessible count
        self.buffer_accessible
            .as_ref()
            .unwrap()
            .write(&vec![0i32])
            .enq()
            .map_err(|_| UpdateError::InvalidInput)?;

        // Update kernel args to point to correct buffers
        let kernel = self.kernel_cl.as_mut().unwrap();
        kernel
            .set_arg("input", input_buf.as_ref().unwrap())
            .unwrap();
        kernel
            .set_arg("output", output_buf.as_ref().unwrap())
            .unwrap();

        unsafe {
            kernel.enq().map_err(|_| UpdateError::InvalidInput)?;
        }

        // Read accessible count
        let mut accessible = vec![0i32; 1];
        self.buffer_accessible
            .as_ref()
            .unwrap()
            .read(&mut accessible)
            .enq()
            .map_err(|_| UpdateError::InvalidInput)?;

        Ok(accessible[0] as u32)
    }
}

impl Default for Day4Solver {
    fn default() -> Self {
        // Kernel that counts neighbors
        // 1/8 1/8 1/8
        // 1/8 0   1/8
        // 1/8 1/8 1/8
        // Activation 0.5 = 4/8
        Self::new(
            vec![
                1.0 / 8.0,
                1.0 / 8.0,
                1.0 / 8.0,
                1.0 / 8.0,
                0.0,
                1.0 / 8.0,
                1.0 / 8.0,
                1.0 / 8.0,
                1.0 / 8.0,
            ],
            0.5,
        )
    }
}
