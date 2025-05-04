use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();

    // Read a line from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim newline characters
    let input = input.trim();

    let mut map: HashMap<char, i32> = HashMap::new();
    for ch in input.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }

    //println!("{:?}", map);

    let mut n: i32 = input.len() as i32;
    //println!("n = {:?}", n);

    let mut num_permut: i32 = 1;

    for (k, v) in map.iter() {
        //println!("k: {}, v: {}, num_permut: {} -- n: {}", k, v, num_permut, n);

        if n == *v {
            n -= *v;
            continue;
        }

        if *v == 1 {
            num_permut *= n;
            n -= *v;
            continue;
        }

        for i in (n - *v + 1)..=n {
            num_permut *= i;
        }

        //println!("after multiplying... num_permut: {}", num_permut);

        for i in 2..=*v {
            num_permut = num_permut / i;
        }

        //println!("after dividing... num_permut: {}", num_permut);

        n -= v;
    }

    println!("{}", num_permut);
    process_map_and_string(&mut map, "".to_string());


}

fn process_map_and_string(my_map: &mut HashMap<char, i32>, my_str: String) {

	// Your processing logic here using 'my_map' and 'my_string'
    //println!("Received map: {:?}", my_map);
    //println!("Received string: {}", my_str);

	if my_map.len() == 0 {
		println!("{}", my_str);
	}
    
    let mut vec = Vec::new();
    for k in my_map.keys() {
    	vec.push(k.clone());
    }

    for v in vec {
    	update_or_remove_count(my_map, v);
    	//println!("after deleting {} from map: {:?}", v, my_map);
    	let mut str1 = my_str.clone();
    	str1.push(v);
    	process_map_and_string(my_map, str1);
    	increment_map_count(my_map, v);
    	//println!("adding back {} from map: {:?}", v, my_map);
    }

}

fn update_or_remove_count(map: &mut HashMap<char, i32>, key: char) {
    if let Some(count) = map.get_mut(&key) {
        if *count > 1 {
            *count -= 1;

        } else {
            map.remove(&key);
        }
    }
    // If the key doesn't exist, the map remains unchanged.
}

fn increment_map_count(map: &mut HashMap<char, i32>, key: char) {
    if let Some(count) = map.get_mut(&key) {
        *count += 1;
    } else {
        map.insert(key, 1);
    }
}
