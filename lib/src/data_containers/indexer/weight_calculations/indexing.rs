use std::collections::HashMap;

trait IndexingCalculation {
    fn calculate_weights(&mut self, frequencies: HashMap<String, u64>) -> HashMap<String, f32>;
    fn apply_normalization(&self, weight: f32) -> f32;
}

struct Lnc {
    cosine_normalization: f32,
}

impl IndexingCalculation for Lnc {
    fn calculate_weights(&mut self, frequencies: HashMap<String, u64>) -> HashMap<String, f32> {
        let mut weights = HashMap::<String, f32>::with_capacity(frequencies.len());

        let mut weights_square_sum = 0f32;
        for entry in frequencies {
            let term = entry.0;
            let term_frequency = entry.1;

            let term_weight = 1f32 + f32::log10(term_frequency as f32);
            weights.insert(term, term_weight);
            weights_square_sum += f32::powf(term_weight, 2f32)
        }

        self.cosine_normalization = f32::sqrt(weights_square_sum);

        weights
    }

    fn apply_normalization(&self, weight: f32) -> f32 {
        weight / self.cosine_normalization
    }
}
