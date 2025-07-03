/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   strings.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: descamil <descamil@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/07/02 16:07:40 by descamil          #+#    #+#             */
/*   Updated: 2025/07/03 19:07:21 by descamil         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::ascii::is_whitespace;

pub fn reverse(line: &mut String)
{
	let mut chars: Vec<char> = line.chars().collect();
	let len: usize = chars.len();
	for i in 0..len / 2
	{
		chars.swap(i, len - 1 - i);
	}
	*line = chars.into_iter().collect();
}

pub fn to_upper(line: &mut String)
{
	let mut chars: Vec<char> = line.chars().collect();
	for c in &mut chars
	{
		if *c >= 'a' && *c <= 'z'
		{
			*c = ((*c as u8) - 32) as char;
		}
	}
	*line = chars.into_iter().collect();
}

pub fn to_lower(line: &mut String)
{
	let mut chars: Vec<char> = line.chars().collect();
	for c in &mut chars
	{
		if *c >= 'A' && *c <= 'Z'
		{
			*c = ((*c as u8) + 32) as char;
		}
	}
	*line = chars.into_iter().collect();
}

pub fn trim(line: &mut String) 
{
	trim_start(line);
	trim_end(line);
}

pub fn trim_start(line: &mut String)
{
	let chars: Vec<char> = line.chars().collect();

	let start = chars.iter().position(|&c| !is_whitespace(c)).unwrap_or(0);
	
	*line = chars[start..].iter().collect();
}

pub fn trim_end(line: &mut String)
{
	let chars: Vec<char> = line.chars().collect();
	
	let end = chars.iter().rposition(|&c| !is_whitespace(c)).unwrap_or(chars.len() - 1);

	*line = chars[..=end].iter().collect();
}

pub fn start_with(line: &String, prefix: &String) -> bool
{
	let chars: Vec<char> = line.chars().collect();
	let prefix_chars: Vec<char> = prefix.chars().collect();

	if chars.len() < prefix_chars.len()
	{
		return false;
	}
	chars[..prefix_chars.len()] == prefix_chars
}

pub fn end_with(line: &String, suffix: &String) -> bool
{
	let chars: Vec<char> = line.chars().collect();
	let suffix_chars: Vec<char> = suffix.chars().collect();

	if chars.len() < suffix_chars.len()
	{
		return false;
	}
	chars[chars.len() - suffix.len()..] == suffix_chars
}

pub fn contain(line: &String, look: &String) -> bool
{
	let chars: Vec<char> = line.chars().collect();
	let look_chars: Vec<char> = look.chars().collect();

	if chars.len() < look_chars.len()
	{
		return false;
	}
	for i in 0..=chars.len() - look_chars.len()
	{
		if chars[i..look_chars.len() + i] == look_chars
		{
			return true;
		}
	}
	return false;
}