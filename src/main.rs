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

        println!("Enter change rate: ");
        let change_rate: f32 = get_input().parse().expect("Please enter a valid number");

        let mut nets: Vec<Vec<Vec<Vec<f32>>>> = read_nets(change_rate);
        let mut train: (Vec<Vec<Vec<Vec<f32>>>>, Vec<f32>) = (vec![], vec![]);
        let mut scores: Vec<f32> = vec![];

        for _ in 0..epochs {
            // Load input and target data for training
            let targets: Vec<Vec<f32>> = vec![vec![2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 0.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2000.0]];
            //let targets: Vec<Vec<f32>> = vec![vec![100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]];

            // Train the network
            train = train_network2(&nets, &inputs1, &targets, change_rate); // Example learning rate

            nets = train.0;
            scores = train.1;

            nets = eliminate(&nets, scores.clone(), change_rate)
        }

        let mut i = 0;

        for n in nets {
            let _ = write_network_to_file(&n, format!("networks/net{}.json", i).as_str());

            i += 1;
        }
    }

    println!("Please enter path to image: ");
    let image_path = get_input();

    let inputs1: Vec<f32> = image(&image_path);

    let mut ins: Vec<f32> = vec![]; // ins: internal neurons

    for i in 0..129 {
        ins.push(calc_num(&inputs1, &net1[0][i], net1[1][0][i]));
    }

    let mut output: Vec<f32> = vec![];

    for i in 0..9 {
        output.push(calc_num(&ins, &net1[0][i + 130], net1[1][0][i + 130]));
    }

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

fn train_network2(networks: &Vec<Vec<Vec<Vec<f32>>>>, inputs: &Vec<Vec<f32>>, targets: &Vec<Vec<f32>>, learning_rate: f32) -> (Vec<Vec<Vec<Vec<f32>>>>, Vec<f32>) {
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
    let mut net20 = &networks.clone()[19];

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
    let mut output_errors: Vec<Vec<Vec<f32>>> = vec![];
    let mut scores: Vec<f32> = vec![];

    for _ in 0..networks.len() {
        let mut net_output: Vec<Vec<f32>> = vec![];

        for input in inputs {
            let mut ins: Vec<f32> = vec![]; // ins: internal neurons

            for i in 0..129 {
                ins.push(calc_num(&input, &net1[0][i], net1[1][0][i]));
            }
        
            let mut output: Vec<f32> = vec![];
        
            for i in 0..9 {
                output.push(calc_num(&ins, &net1[0][i + 130], net1[1][0][i + 130]));
            }

            net_output.push(output);
        }

        outputs.push(net_output);
    }

    for i in 0..outputs.len() {
        let mut o_errors: Vec<Vec<f32>> = outputs[i].clone();

        for x in 0..outputs[i].len() {
            for y in 0..outputs[i][x].len() {
                o_errors[x][y] = outputs[i][x][y] - targets[x][y];
            }
        }

        output_errors.push(o_errors);
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

    return (vec![(*net1.clone()).to_vec(),(*net2.clone()).to_vec(),(*net3.clone()).to_vec(),(*net4.clone()).to_vec(),(*net5.clone()).to_vec(),(*net6.clone()).to_vec(),(*net7.clone()).to_vec(),(*net8.clone()).to_vec(),(*net9.clone()).to_vec(), (net10.clone()).to_vec(), (net11.clone()).to_vec(), (net12.clone()).to_vec(), (net13.clone()).to_vec(), (net14.clone()).to_vec(), (net15.clone()).to_vec(), (net16.clone()).to_vec(), (net17.clone()).to_vec(), (net18.clone()).to_vec(), (net19.clone()).to_vec(), (net20.clone()).to_vec()], scores);
}

fn eliminate(networks: &Vec<Vec<Vec<Vec<f32>>>>, scores: Vec<f32>, change_rate: f32) -> Vec<Vec<Vec<Vec<f32>>>> {
    let mut network_score_pairs: Vec<(&Vec<Vec<Vec<f32>>>, f32)> = 
        networks.iter().zip(scores.iter()).map(|(network, &score)| (network, score)).collect();

    // Sort the vector of tuples by the scores
    network_score_pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let sorted_networks: Vec<&Vec<Vec<Vec<f32>>>> = network_score_pairs.iter().map(|&(network, _)| network).collect();

    let mut nets = networks.clone();

    nets[0] = (sorted_networks[16].clone()).to_vec();
    nets[1] = (sorted_networks[17].clone()).to_vec();
    nets[2] = (sorted_networks[18].clone()).to_vec();
    nets[3] = (sorted_networks[19].clone()).to_vec();

    nets[4] = change_net(&sorted_networks[0][1][0], &sorted_networks[16][0], change_rate);
    nets[5] = change_net(&sorted_networks[0][1][0], &sorted_networks[16][0], change_rate);
    nets[6] = change_net(&sorted_networks[0][1][0], &sorted_networks[16][0], change_rate);
    nets[7] = change_net(&sorted_networks[0][1][0], &sorted_networks[16][0], change_rate);

    nets[8] = change_net(&sorted_networks[1][1][0], &sorted_networks[17][0], change_rate);
    nets[9] = change_net(&sorted_networks[1][1][0], &sorted_networks[17][0], change_rate);
    nets[10] = change_net(&sorted_networks[1][1][0], &sorted_networks[17][0], change_rate);
    nets[11] = change_net(&sorted_networks[1][1][0], &sorted_networks[17][0], change_rate);
    
    nets[12] = change_net(&sorted_networks[2][1][0], &sorted_networks[18][0], change_rate);
    nets[13] = change_net(&sorted_networks[2][1][0], &sorted_networks[18][0], change_rate);
    nets[14] = change_net(&sorted_networks[2][1][0], &sorted_networks[18][0], change_rate);
    nets[15] = change_net(&sorted_networks[2][1][0], &sorted_networks[18][0], change_rate);
    
    nets[16] = change_net(&sorted_networks[3][1][0], &sorted_networks[19][0], change_rate);
    nets[17] = change_net(&sorted_networks[3][1][0], &sorted_networks[19][0], change_rate);
    nets[18] = change_net(&sorted_networks[3][1][0], &sorted_networks[19][0], change_rate);
    nets[19] = change_net(&sorted_networks[3][1][0], &sorted_networks[19][0], change_rate);
    
    nets
}

fn read_nets(change_rate: f32) -> Vec<Vec<Vec<Vec<f32>>>> {
    return vec![change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), change_net(&read_network_from_file("networks/net")[1][0], &read_network_from_file("networks/net")[0], change_rate), ];
}