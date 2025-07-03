/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ascii.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: descamil <descamil@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/07/02 16:07:25 by descamil          #+#    #+#             */
/*   Updated: 2025/07/02 17:47:50 by descamil         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn is_ascii_alpha(c: char) -> bool
{
	(c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
}

pub fn is_ascii_digit(c: char) -> bool
{
	c >= '0' && c <= '9'
}

pub fn is_ascii_alphanumeric(c: char) -> bool
{
	is_ascii_digit(c) || is_ascii_alpha(c)
}

pub fn is_whitespace(c: char) -> bool
{
	c == ' '
}