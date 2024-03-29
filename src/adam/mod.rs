// READ NOTE IN GD OBJ MOD

// ADAM is something that actually does need to be implemented, so in the future this code will be fixed to work with the 
// neural network refactors. For now, it will be commented out to avoid compiler errors.

/* 
pub mod adam {

    /// Adam optimization algorithm implementation
    /// Src: https://arxiv.org/abs/1412.6980

    use crate::gradient_descent::obj::obj::GradientDescent;
    use crate::nn::nn::nn::NeuralNetwork;
    //use std::borrow::Borrow;
    //use crate::batch;
    //use crate::batch_vectorized;
    //use crate::stochastic_vect;
    
    pub struct Adam {
        nn: NeuralNetwork,
        gd: GradientDescent,
        pub stepsize: f64,
        pub beta_1: f64,
        pub beta_2: f64,
        pub epsilon: f64,
        pub m_b: Vec<f64>,
        pub v_b: Vec<f64>
    }

    impl Adam {

        /// Creates a new Adam optimizer object. If default is passed as true,
        /// then the base coefficients for the stepsize and exponential decay rates are set to
        /// default, being alpha = 0.0001, beta 1 = 0.9, beta 2 = 0.999, and epsilon = 1e-8
        /// When not using default coefficients, the new function can intake values for 
        /// each of these parameters
        ///
        /// ### Parameters
        /// 
        /// a: stepsize
        /// 
        /// beta_1 & beta_2: exponential decay rates for the moment estimates
        /// 
        /// epsilon: small constant used to avoid division by zero when calculating RMS of the gradients
        pub fn new(
            nn: NeuralNetwork,
            gd: GradientDescent,
            default: bool,
            stepsize: Option<f64>,
            beta_1: Option<f64>,
            beta_2: Option<f64>,
            epsilon: Option<f64>,
        ) -> Adam {
            let (stepsize, beta_1, beta_2, epsilon) = if default {
                (
                    Some(0.0001),
                    Some(0.9),
                    Some(0.999),
                    Some(1e-8),
                )
            } else {
                (stepsize, beta_1, beta_2, epsilon)
            };
            let stepsize: f64 = stepsize.unwrap_or_else(|| {
                println!("Warning: No stepsize provided; using default value.");
                0.0001
            });
            let beta_1 = beta_1.unwrap_or_else(|| {
                println!("Warning: No beta_1 provided; using default value.");
                0.9
            });
            let beta_2 = beta_2.unwrap_or_else(|| {
                println!("Warning: No beta_2 provided; using default value.");
                0.999
            });
            let epsilon = epsilon.unwrap_or_else(|| {
                println!("Warning: No epsilon provided; using default value.");
                1e-8
            });
            println!(
                "Loading Adam parameters: alpha: {}, beta 1: {}, beta 2: {}, epsilon: {}",
                stepsize, beta_1, beta_2, epsilon
            );
            let gd_clone: GradientDescent = gd.clone();
            Adam {
                nn,
                gd,
                stepsize,
                beta_1,
                beta_2,
                epsilon,
                m_b: vec![0.0; gd_clone.num_predictors],
                v_b: vec![0.0; gd_clone.num_predictors]
            }
        }

        pub fn optimize(&mut self, epochs: usize) {
            println!("\nADAM OPTIMIZING");
            let m: f64 = self.gd.x_train.len() as f64;
            //println!("Adam m: {}", m.clone());
            let mut gd = self.gd.clone();
            let mut gradients: Vec<f64> = Vec::new();
            for epoch in 0..epochs {
                for (predictor, output) in self.gd.train_data() {
                    //println!("Adam predictor: {:?}", predictor.clone());
                    //println!("Corresponding output: {}", output.clone());
                    let mut error: f64 = self.gd.h(predictor.clone()) - output;
                    if error.is_sign_negative() {
                        error *= -1.0;
                    }
                    //println!("Adam Initial Err: {}", error.clone());
                    /*let gradients: Vec<f64> = predictor
                        .iter()
                        .enumerate()
                        .map(|(j, &x)| (self.gd.learning_rate / m) * error * x)
                        .collect();
                    */
                    for (_j, &x) in predictor.iter().enumerate() {
                        gradients.push((self.gd.learning_rate / m) * error * x);
                    }
                    //println!("Initial Adam Gradients Vector: {:?}", gradients.clone());
                }
                //println!("Adam Gradients: {:?}", &gradients);
                gd.adam_update(self, &gradients, epoch);
                //println!(
                //    "Epoch: {}, Theta Vector: {:#?}, Bias: {}, Cost: {:?}",
                //    epoch,
                //    gd.theta_matrix,
                //    gd.b,
                //    gd.cost(gd.theta_matrix.clone(), gd.b)
                //);
                self.gd.update_neural_net()
            }
        }
    }
}
*/