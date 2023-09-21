#![allow(unused)]
use super::ShootRes;
use plotters::{prelude::*, data::float};

pub struct ResultAnalyzer {
    // This struct will recieve the Shoot result vector to process into an image output,
    // file output or Text based output (by default)
    shoot_res_vec: Vec<ShootRes>,
    shoot_res_err_vec: Vec<ShootRes>,
}

impl ResultAnalyzer {
    // Consume this since later on we will call from this object instead
    pub fn new(mut shoot_res_vec: Vec<ShootRes>) -> Self {
        shoot_res_vec.sort_by(|a, b| a.timestamp.cmp(&b.timestamp)); // Sort each request by timestamp
        ResultAnalyzer {
            shoot_res_vec: shoot_res_vec,
            shoot_res_err_vec: Vec::new(),
        }
    }

    pub fn split_err_event(&mut self) -> Result<usize, String> {
        if self.shoot_res_vec.len() == 0 {
            Err(String::from("Nothing to split"))
        } else {
            let item_count = self.shoot_res_vec.len();
            for i in 0..item_count {
                if self.shoot_res_vec.get(i).unwrap().is_err() {
                    self.shoot_res_err_vec.push(self.shoot_res_vec.remove(i));
                }
            }
            Ok(item_count)
        }
    }

    pub fn is_err_splitted(&self) -> bool {
        return self.shoot_res_err_vec.len() > 0;
    }

    pub fn merge_back_err_event(&mut self) -> Result<usize, String> {
        let mut merge_count: usize = 0;
        if self.shoot_res_err_vec.len() == 0 {
            Err(String::from("Nothing to merge"))
        } else if self.shoot_res_vec.len() == 0 {
            // If nothing
            while let Some(e_shr) = self.shoot_res_err_vec.pop() {
                merge_count += 1;
                self.shoot_res_vec.push(e_shr);
            }
            Ok(merge_count)
        } else {
            // Since I have no idea how to merge vectors using first element so I decided to invert all logic.
            let mut pos_shoot_res_vec: usize = self.shoot_res_vec.len() - 1;
            while let Some(err_shr) = self.shoot_res_err_vec.pop() {
                if pos_shoot_res_vec > self.shoot_res_vec.len() - 1 {
                    // Same condition since we expect to jump to overflow
                    self.shoot_res_vec.insert(0, err_shr);
                    merge_count += 1;
                    break;
                } else {
                    while let Some(succ_shr) = self.shoot_res_vec.get(pos_shoot_res_vec) {
                        if err_shr.timestamp > succ_shr.timestamp {
                            self.shoot_res_vec.insert(pos_shoot_res_vec + 1, err_shr);
                            merge_count += 1;
                            break;
                        } else {
                            pos_shoot_res_vec -= 1;
                        }
                    }
                }
            }
            Ok(merge_count)
        }
    }

    pub fn build_latency_vectors(&self) -> Vec<(u128, u128)> {
        let ts_offset = match self.shoot_res_vec.get(0) {
            Some(first_ev) => first_ev.timestamp,
            None => 0,
        };
        let mut lat_vec: Vec<(u128, u128)> = Vec::new();
        for each_succ_shoot in &self.shoot_res_vec {
            match each_succ_shoot.latency {
                Some(lat) => {
                    lat_vec.push((each_succ_shoot.timestamp - ts_offset, lat));
                }
                None => {
                    lat_vec.push((each_succ_shoot.timestamp - ts_offset, 0));
                },
            }
        }
        lat_vec
    }

    pub fn show_latencies(&self) {
        for event in self.build_latency_vectors() {
            println!("[{}]: {} ms",event.0, event.1);
        }
    }

    pub fn plot_latency(&self, file_loc: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Scale the chart by searching for the min/max x and y
        let data = self.build_latency_vectors();
        let max_x = {match data.last() {
            Some(dat) => dat.0,
            None => 0,
        }};
        let max_y = {
            let mut mx_y = 0;
            for dat in &data {
                if mx_y < dat.1 {
                    mx_y = dat.1;
                }
            }
            mx_y
        };
        let min_y = {
            let mut mn_y = match data.get(0) {
                Some(dat) => dat.1,
                None => 0,
            };
            for dat in &data {
                if mn_y > dat.1 {
                    mn_y = dat.1;
                }
            }
            mn_y
        };

        let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> = BitMapBackend::new(file_loc, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart_builder = ChartBuilder::on(&root);
        
        let mut chart = chart_builder
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .caption("Latency", ("sans-serif", 40))
        .build_cartesian_2d(0..max_x, min_y..max_y)?;

        chart.configure_mesh().draw()?;

        chart.draw_series(
            AreaSeries::new(
                data,
                0,
                &RED.mix(0.2),
            )
            .border_style(&RED),
        )?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        Ok(format!("Result has been saved to {}", file_loc))

    }
}
