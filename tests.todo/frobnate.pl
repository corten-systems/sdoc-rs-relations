#!/usr/bin/env perl
use strict;
use warnings;

# Randomly transform @relation(identifier) to @relation(identifier, key=value, key=value), where
# the keys and values are random alphanumeric strings and valid spaces are randomly inserted.
#
# Usage: frobnate.pl < input.rs > output.rs


# Function to generate random alphanumeric string
sub rand_alnum {
    my $len = shift || int(rand(5)) + 3;  # 3-7 chars
    my @chars = ('a'..'z', 'A'..'Z', '0'..'9');
    return join '', map { $chars[rand @chars] } 1..$len;
}

# Function to generate random spaces (0, 1, or 2)
sub rand_spaces {
    return ' ' x int(rand(3));
}

while (my $line = <STDIN>) {
    # Match @relation (with optional spaces) (identifier) where identifier is alphanumeric
    $line =~ s/\@relation\s*\((\w+)\)/transform_relation($1)/ge;
    print $line;
}

sub transform_relation {
    my $id = shift;
    
    # Randomly choose 0, 1, or 2 key-value pairs
    my $num_pairs = int(rand(3));
    
    my @pairs;
    for (1..$num_pairs) {
        my $key = rand_alnum();
        my $value = rand_alnum();
        push @pairs, "$key=$value";
    }
    
    # Build the replacement string with random spacing
    my $result = "\@relation" . rand_spaces() . "(" . rand_spaces() . $id;
    
    if (@pairs) {
        foreach my $pair (@pairs) {
            $result .= rand_spaces() . ',' . rand_spaces() . $pair;
        }
    }
    
    $result .= rand_spaces() . ')';
    
    return $result;
}
