/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: descamil <descamil@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/07/02 16:07:44 by descamil          #+#    #+#             */
/*   Updated: 2025/07/03 19:05:19 by descamil         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod ascii;
mod strings;

fn main()
{
	let mut text: String = String::from("    Hello    ");
	
	println!("{:<10}{}", "Text:", text);
	strings::trim(&mut text);
	println!("{:<10}{}", "Trim:", text);

	strings::to_upper(&mut text);
	println!("{:<10}{}", "Upper:", text);
	strings::reverse(&mut text);
	println!("{:<10}{}", "Reverse:", text);
	strings::to_lower(&mut text);
	println!("{:<10}{}", "Lower:", text);
	strings::reverse(&mut text);
	println!("{:<10}{}", "Reverse:", text);
	println!();

	let mut contain1: bool = strings::start_with( &text,  &String::from("he"));
	let mut contain2: bool = strings::start_with( &text,  &String::from("lo"));
	println!("Prefix: \"he\" in {} --> {}", &text, &contain1);
	println!("Prefix: \"lo\" in {} --> {}", &text, &contain2);
	contain1 = strings::end_with( &text,  &String::from("he"));
	contain2 = strings::end_with( &text,  &String::from("lo"));
	println!("Suffix: \"he\" in {} --> {}", text, contain1);
	println!("Suffix: \"lo\" in {} --> {}", text, contain2);
	println!();

	contain1 = strings::contain( &text,  &String::from("llo"));
	contain2 = strings::contain( &text,  &String::from("hol"));
	println!("Contain: \"llo\" in {} --> {}", text, contain1);
	println!("Contain: \"hol\" in {} --> {}", text, contain2);
	println!();


	let test_chars: [char; 5] = ['A', '1', ' ', 'z', '?'];

	for &c in &test_chars
	{
		println!("Char '{}': is_ascii_alpha: {:<7}, is_ascii_digit: {:<7}, is_ascii_alphanumeric: {:<7}, is_whitespace: {:<7}", c,
			ascii::is_ascii_alpha(c), ascii::is_ascii_digit(c), ascii::is_ascii_alphanumeric(c), ascii::is_whitespace(c));
	}
}
