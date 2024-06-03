use rand::Rng;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

pub fn main() {
    let mut net1: Vec<Vec<Vec<f32>>> = vec![];

    println!("Do you want to load a network? (y, n) : ");
    let load = get_input();

    if load == "y" {
        println!("What is the path to the network?: ");
        let net_path = get_input();
        net1 = read_network_from_file(&net_path);
    }
    else {
        net1 = read_network_from_file("networks/net");
    }

    println!("What would you like to do? (p: print net, b: backup network, t: train network) : ");
    let response = get_input(); 

    if response == "p" {
        println!("{:?}", net1);
    }

    if response == "b" {
        println!("What is the path to the file you want to save the network to?: ");
        let save_path = get_input();
        write_network_to_file(&net1, &save_path).expect("Failed to write network to file");
    }

    if response == "t" {
        println!("Enter number of training epochs: ");
        let epochs: usize = get_input().parse().expect("Please enter a valid number");

        let inputs1: Vec<Vec<f32>> = vec![image(&"numbers/one1.png".to_string()), image(&"numbers/one2.png".to_string()), image(&"numbers/one3.png".to_string()), image(&"numbers/one4.png".to_string()), image(&"numbers/one5.png".to_string()), image(&"numbers/two1.png".to_string()), image(&"numbers/two2.png".to_string()),image(&"numbers/three1.png".to_string()), image(&"numbers/three2.png".to_string()), image(&"numbers/three3.png".to_string()), image(&"numbers/three4.png".to_string()), image(&"numbers/three5.png".to_string()), image(&"numbers/four1.png".to_string()), image(&"numbers/four2.png".to_string()), image(&"numbers/four3.png".to_string()), image(&"numbers/four4.png".to_string()), image(&"numbers/four5.png".to_string()), image(&"numbers/five1.png".to_string()), image(&"numbers/five2.png".to_string()), image(&"numbers/five3.png".to_string()), image(&"numbers/five4.png".to_string()), image(&"numbers/five5.png".to_string()), image(&"numbers/six1.png".to_string()), image(&"numbers/six2.png".to_string()), image(&"numbers/six3.png".to_string()), image(&"numbers/six4.png".to_string()), image(&"numbers/six5.png".to_string()), image(&"numbers/seven1.png".to_string()), image(&"numbers/seven2.png".to_string()), image(&"numbers/seven3.png".to_string()), image(&"numbers/seven4.png".to_string()), image(&"numbers/seven5.png".to_string()), image(&"numbers/eight1.png".to_string()), image(&"numbers/eight2.png".to_string()), image(&"numbers/eight3.png".to_string()), image(&"numbers/eight4.png".to_string()), image(&"numbers/eight5.png".to_string()), image(&"numbers/nine1.png".to_string()), image(&"numbers/nine2.png".to_string()), image(&"numbers/nine3.png".to_string()), image(&"numbers/nine4.png".to_string()), image(&"numbers/nine5.png".to_string()), image(&"numbers/zero1.png".to_string()), image(&"numbers/zero2.png".to_string()), image(&"numbers/zero3.png".to_string()), image(&"numbers/zero4.png".to_string()), image(&"numbers/zero5.png".to_string())];
        //let inputs1: Vec<Vec<f32>> = vec![image(&"numbers/one1.png".to_string())];

        let mut change_logs: Vec<String> = vec![];
        let mut train: (Vec<Vec<Vec<f32>>>, Vec<String>);

        for _ in 0..epochs {
            // Load input and target data for training
            let targets: Vec<Vec<f32>> = vec![vec![100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0]]; // Replace with actual target data
            //let targets: Vec<Vec<f32>> = vec![vec![100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]];

            // Train the network
            train = train_network(&net1, &inputs1, &targets, 0.000001, &change_logs); // Example learning rate

            net1 = train.0;
            change_logs = train.1;
        }

        println!("Enter net save path: ");
        let save_path = get_input();

        write_network_to_file(&net1, &save_path).expect("Failed to write network to file");
    }

    println!("Please enter path to image: ");
    let image_path = get_input();

    let inputs1: Vec<f32> = image(&image_path);

    let n1 = calc_num(&inputs1, &net1[0][0], net1[1][0][0]);
    let n2 = calc_num(&inputs1, &net1[0][1], net1[1][0][1]);
    let n3 = calc_num(&inputs1, &net1[0][2], net1[1][0][2]);
    let n4 = calc_num(&inputs1, &net1[0][3], net1[1][0][3]);
    let n5 = calc_num(&inputs1, &net1[0][4], net1[1][0][4]);
    let n6 = calc_num(&inputs1, &net1[0][5], net1[1][0][5]);
    let n7 = calc_num(&inputs1, &net1[0][6], net1[1][0][6]);
    let n8 = calc_num(&inputs1, &net1[0][7], net1[1][0][7]);
    let n9 = calc_num(&inputs1, &net1[0][8], net1[1][0][8]);
    let n10 = calc_num(&inputs1, &net1[0][9], net1[1][0][9]);
    let n11 = calc_num(&inputs1, &net1[0][10], net1[1][0][10]);
    let n12 = calc_num(&inputs1, &net1[0][11], net1[1][0][11]);
    let n13 = calc_num(&inputs1, &net1[0][12], net1[1][0][12]);
    let n14 = calc_num(&inputs1, &net1[0][13], net1[1][0][13]);
    let n15 = calc_num(&inputs1, &net1[0][14], net1[1][0][14]);

    let ins: Vec<f32> = vec![n1, n2, n3, n4, n5, n6, n7, n8, n9, n10, n11, n12, n13, n14, n15]; // ins: internal neurons

    let n16 = calc_num(&ins, &net1[0][15], net1[1][0][15]);
    let n17 = calc_num(&ins, &net1[0][16], net1[1][0][16]);
    let n18 = calc_num(&ins, &net1[0][17], net1[1][0][17]);
    let n19 = calc_num(&ins, &net1[0][18], net1[1][0][18]);
    let n20 = calc_num(&ins, &net1[0][19], net1[1][0][19]);
    let n21 = calc_num(&ins, &net1[0][20], net1[1][0][20]);
    let n22 = calc_num(&ins, &net1[0][21], net1[1][0][21]);
    let n23 = calc_num(&ins, &net1[0][22], net1[1][0][22]);
    let n24 = calc_num(&ins, &net1[0][23], net1[1][0][23]);
    let n25 = calc_num(&ins, &net1[0][24], net1[1][0][24]);

    let output: Vec<f32> = vec![n16, n17, n18, n19, n20, n21, n22, n23, n24, n25];

    println!("{:?}", output);
}

fn calc_num(nums: &Vec<f32>, wghts: &Vec<f32>, bias: f32) -> f32 {
    let mut wednums: Vec<f32> = vec![];
    let mut newnum = 0.0;

    for (i, x) in nums.iter().enumerate() {
        wednums.push(x * wghts[i]);
    }

    for x in &wednums {
        newnum += x;
    }

    newnum += bias;

    if newnum < 0.0 {
        newnum = 0.0;
    }

    return newnum;
}

fn change_net(bias: &Vec<f32>, wghts: &Vec<Vec<f32>>, change_rate: f32) -> Vec<Vec<Vec<f32>>> {
    let mut rng = rand::thread_rng();

    let mut nbias = bias.clone();
    let mut nwghts = wghts.clone();

    let mut i = 0;

    for _ in bias {
        nbias[i] += rng.gen_range(-change_rate..=change_rate);

        i += 1;
    }

    i = 0;

    for _ in wghts {
        let mut i2 = 0;

        for _ in wghts[i].clone() {
            nwghts[i][i2] += rng.gen_range(-change_rate..=change_rate);
            i2 += 1;
        }

        i += 1;
    }

    let rvalue: Vec<Vec<Vec<f32>>> = vec![nwghts, vec![nbias]];

    return rvalue;
}

pub fn image(image_path: &String) -> Vec<f32> {
    // Open the image
    let img = image::open(image_path).expect("Failed to open image");

    // Convert the image to grayscale
    let grayscale_img = img.to_luma8();

    // Convert the grayscale image to a flat vector of 1.0s and 0.0s
    let binary_representation = convert_to_binary(&grayscale_img);

    binary_representation
}

fn convert_to_binary(img: &image::GrayImage) -> Vec<f32> {
    img.pixels().map(|p| p.0[0] as f32 / 255.0).collect()
}


fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn write_network_to_file(network: &Vec<Vec<Vec<f32>>>, file_path: &str) -> io::Result<()> {
    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &network)?;
    Ok(())
}

fn read_network_from_file(file_path: &str) -> Vec<Vec<Vec<f32>>> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let network: Vec<Vec<Vec<f32>>> = serde_json::from_reader(reader).expect("Failed to read network from file");
    network
}

fn train_network(network: &Vec<Vec<Vec<f32>>>, inputs: &Vec<Vec<f32>>, targets: &Vec<Vec<f32>>, learning_rate: f32, change_logs: &Vec<String>) -> (Vec<Vec<Vec<f32>>>, Vec<String>) {
    let mut net = network.clone();
    let logs = change_logs.clone();

    for x in 0..inputs.len() {
        let input = &inputs[x];
        let target = &targets[x];

        // Forward pass
        let mut hidden_outputs: Vec<f32> = vec![];
        for i in 0..15 {
            hidden_outputs.push(calc_num(input, &net[0][i], net[1][0][i]));
        }

        let mut final_outputs: Vec<f32> = vec![];
        for i in 15..25 {
            final_outputs.push(calc_num(&hidden_outputs, &net[0][i], net[1][0][i]));
        }

        // Calculate output errors
        let mut output_errors: Vec<f32> = vec![];
        for i in 0..target.len() {
            output_errors.push(target[i] - final_outputs[i]);
        }

        // Update weights and biases for the output layer
        for i in 15..25 {
            for j in 0..hidden_outputs.len() {
                net[0][i][j] += learning_rate * output_errors[i - 15] * hidden_outputs[j];
                //logs.push(format!("output layer weights update: net[0][{}][{}] += {} * {} * {}", i.to_string(), j.to_string(), learning_rate.to_string(), output_errors[i - 15].to_string(), hidden_outputs[j].to_string()));
            }
            net[1][0][i] += learning_rate * output_errors[i - 15];
            //logs.push(format!("output layer biases update: net[1][0][{}], += {} * {}", i.to_string(), learning_rate, output_errors[i - 15]));
        }

        // Calculate hidden layer errors
        let mut hidden_errors: Vec<f32> = vec![0.0; 15];
        for i in 0..hidden_errors.len() {
            for j in 15..25 {
                hidden_errors[i] += output_errors[j - 15] * net[0][j][i];
            }
        }

        // Update weights and biases for the hidden layer
        for i in 0..15 {
            for j in 0..input.len() {
                net[0][i][j] += learning_rate * hidden_errors[i] * input[j];
                //logs.push(format!("hidden layer weights update: net[0][{}][{}] += {} * {} * {}", i.to_string(), j.to_string(), learning_rate.to_string(), hidden_errors[i].to_string(), input[j].to_string()));
            }
            net[1][0][i] += learning_rate * hidden_errors[i];
            //logs.push(format!("hidden layer biases update: net[1][0][{}] += {} * {}", i.to_string(), learning_rate.to_string(), hidden_errors[i].to_string()));
        }
    }

    (net, logs)
}

fn train_network2(networks: &Vec<Vec<Vec<Vec<f32>>>>, inputs: &Vec<Vec<f32>>, targets: &Vec<Vec<f32>>, learning_rate: f32) {
    let mut net1 = &networks.clone()[0];
    let mut net2 = &networks.clone()[1];
    let mut net3 = &networks.clone()[2];
    let mut net4 = &networks.clone()[3];
    let mut net5 = &networks.clone()[4];
    let mut net6 = &networks.clone()[5];
    let mut net7 = &networks.clone()[6];
    let mut net8 = &networks.clone()[7];
    let mut net9 = &networks.clone()[8];
    let mut net10 = &networks.clone()[9];
    let mut net11 = &networks.clone()[10];
    let mut net12 = &networks.clone()[11];
    let mut net13 = &networks.clone()[12];
    let mut net14 = &networks.clone()[13];
    let mut net15 = &networks.clone()[14];
    let mut net16 = &networks.clone()[15];
    let mut net17 = &networks.clone()[16];
    let mut net18 = &networks.clone()[17];
    let mut net19 = &networks.clone()[18];
    let mut net20 = &networks.clone()[29];
    
    let _ = write_network_to_file(&net1, "networks/net1.json");
    let _ = write_network_to_file(&net2, "networks/net2.json");
    let _ = write_network_to_file(&net3, "networks/net3.json");
    let _ = write_network_to_file(&net4, "networks/net4.json");
    let _ = write_network_to_file(&net5, "networks/net5.json");
    let _ = write_network_to_file(&net6, "networks/net6.json");
    let _ = write_network_to_file(&net7, "networks/net7.json");
    let _ = write_network_to_file(&net8, "networks/net8.json");
    let _ = write_network_to_file(&net9, "networks/net9.json");
    let _ = write_network_to_file(&net10, "networks/net10.json");
    let _ = write_network_to_file(&net11, "networks/net11.json");
    let _ = write_network_to_file(&net12, "networks/net12.json");
    let _ = write_network_to_file(&net13, "networks/net13.json");
    let _ = write_network_to_file(&net14, "networks/net14.json");
    let _ = write_network_to_file(&net15, "networks/net15.json");
    let _ = write_network_to_file(&net16, "networks/net16.json");
    let _ = write_network_to_file(&net17, "networks/net17.json");
    let _ = write_network_to_file(&net18, "networks/net18.json");
    let _ = write_network_to_file(&net19, "networks/net19.json");
    let _ = write_network_to_file(&net20, "networks/net20.json");

    let net1_clone = change_net(&net1[1][0], &net1[0], learning_rate);
    net1 = &net1_clone;
    let net2_clone = change_net(&net2[1][0], &net2[0], learning_rate);
    net2 = &net2_clone;
    let net3_clone = change_net(&net3[1][0], &net3[0], learning_rate);
    net3 = &net3_clone;
    let net4_clone = change_net(&net4[1][0], &net4[0], learning_rate);
    net4 = &net4_clone;
    let net5_clone = change_net(&net5[1][0], &net5[0], learning_rate);
    net5 = &net5_clone;
    let net6_clone = change_net(&net6[1][0], &net6[0], learning_rate);
    net6 = &net6_clone;
    let net7_clone = change_net(&net7[1][0], &net7[0], learning_rate);
    net7 = &net7_clone;
    let net8_clone = change_net(&net8[1][0], &net8[0], learning_rate);
    net8 = &net8_clone;
    let net9_clone = change_net(&net9[1][0], &net9[0], learning_rate);
    net9 = &net9_clone;
    let net10_clone = change_net(&net10[1][0], &net10[0], learning_rate);
    net10 = &net10_clone;
    let net11_clone = change_net(&net11[1][0], &net11[0], learning_rate);
    net11 = &net11_clone;
    let net12_clone = change_net(&net12[1][0], &net12[0], learning_rate);
    net12 = &net12_clone;
    let net13_clone = change_net(&net13[1][0], &net13[0], learning_rate);
    net13 = &net13_clone;
    let net14_clone = change_net(&net14[1][0], &net14[0], learning_rate);
    net14 = &net14_clone;
    let net15_clone = change_net(&net15[1][0], &net15[0], learning_rate);
    net15 = &net15_clone;
    let net16_clone = change_net(&net16[1][0], &net16[0], learning_rate);
    net16 = &net16_clone;
    let net17_clone = change_net(&net17[1][0], &net17[0], learning_rate);
    net17 = &net17_clone;
    let net18_clone = change_net(&net18[1][0], &net18[0], learning_rate);
    net18 = &net18_clone;
    let net19_clone = change_net(&net19[1][0], &net19[0], learning_rate);
    net19 = &net19_clone;
    let net20_clone = change_net(&net20[1][0], &net20[0], learning_rate);
    net20 = &net20_clone;
    
    let networks: Vec<Vec<Vec<Vec<f32>>>> = vec![net1.clone(), net2.clone(), net3.clone(), net4.clone(), net5.clone(), net6.clone(), net7.clone(), net8.clone(), net9.clone(), net10.clone(), net11.clone(), net12.clone(), net13.clone(), net14.clone(), net15.clone(), net16.clone(), net17.clone(), net18.clone(), net19.clone(), net20.clone()];
    let mut outputs: Vec<Vec<Vec<f32>>> = vec![];
    let mut output_errors: Vec<Vec<Vec<f32>>> = vec![]; // write to output errors
    let mut scores: Vec<f32> = vec![];

    for _ in 0..networks.len() {
        let mut net_output: Vec<Vec<f32>> = vec![];

        for input in inputs {
            let n1 = calc_num(&input, &net1[0][0], net1[1][0][0]);
            let n2 = calc_num(&input, &net1[0][1], net1[1][0][1]);
            let n3 = calc_num(&input, &net1[0][2], net1[1][0][2]);
            let n4 = calc_num(&input, &net1[0][3], net1[1][0][3]);
            let n5 = calc_num(&input, &net1[0][4], net1[1][0][4]);
            let n6 = calc_num(&input, &net1[0][5], net1[1][0][5]);
            let n7 = calc_num(&input, &net1[0][6], net1[1][0][6]);
            let n8 = calc_num(&input, &net1[0][7], net1[1][0][7]);
            let n9 = calc_num(&input, &net1[0][8], net1[1][0][8]);
            let n10 = calc_num(&input, &net1[0][9], net1[1][0][9]);
            let n11 = calc_num(&input, &net1[0][10], net1[1][0][10]);
            let n12 = calc_num(&input, &net1[0][11], net1[1][0][11]);
            let n13 = calc_num(&input, &net1[0][12], net1[1][0][12]);
            let n14 = calc_num(&input, &net1[0][13], net1[1][0][13]);
            let n15 = calc_num(&input, &net1[0][14], net1[1][0][14]);

            let ins: Vec<f32> = vec![n1, n2, n3, n4, n5, n6, n7, n8, n9, n10, n11, n12, n13, n14, n15]; // ins: internal neurons

            let n16 = calc_num(&ins, &net1[0][15], net1[1][0][15]);
            let n17 = calc_num(&ins, &net1[0][16], net1[1][0][16]);
            let n18 = calc_num(&ins, &net1[0][17], net1[1][0][17]);
            let n19 = calc_num(&ins, &net1[0][18], net1[1][0][18]);
            let n20 = calc_num(&ins, &net1[0][19], net1[1][0][19]);
            let n21 = calc_num(&ins, &net1[0][20], net1[1][0][20]);
            let n22 = calc_num(&ins, &net1[0][21], net1[1][0][21]);
            let n23 = calc_num(&ins, &net1[0][22], net1[1][0][22]);
            let n24 = calc_num(&ins, &net1[0][23], net1[1][0][23]);
            let n25 = calc_num(&ins, &net1[0][24], net1[1][0][24]);

            let output: Vec<f32> = vec![n16, n17, n18, n19, n20, n21, n22, n23, n24, n25];

            net_output.push(output);
        }

        outputs.push(net_output);
    }

    let mut prescore: Vec<Vec<Vec<f32>>> = output_errors.clone();
    let mut prescorevecs: Vec<Vec<f32>> = vec![];

    for i in 0..output_errors.len() {
        for x in 0..output_errors[i].len() {
            let mut y = 0;
            let mut target_score: Vec<f32> = vec![0.0];

            for j in &output_errors[i][x] {
                if *j < 0.0 {
                    prescore[i][x][y] = j * 2.0;
                }

                target_score[0] += prescore[i][x][y];

                y += 1;
            }

            prescorevecs.push(target_score);
        }
    }

    for i in prescorevecs {
        let mut target_score = 0.0;

        for x in i {
            target_score += x;
        }

        scores.push(target_score);
    }
}