use std::collections::HashSet;
/* Author: Arian Seyediarzilpour
** Attention: All kinds of ties are also acceptable inputs as all cases have been accounted for.
** However, in case of identical hands, default winner is assumed to be hand 1.
*/
fn main() {
	println!("\n***-----------Testing winner begins:------------***\n");
	let mut rank1_w = [(10,"C".to_string()), (11,"C".to_string()), (12,"C".to_string()), (13,"C".to_string()),(14,"C".to_string())];

	let mut rank2_w = [(7,"C".to_string()), (8,"C".to_string()), (9,"C".to_string()), (10,"C".to_string()),(11,"C".to_string())];
	let mut rank2_l = [(6,"C".to_string()), (8,"C".to_string()), (7,"C".to_string()), (5,"C".to_string()),(9,"C".to_string())];
	
	let mut rank3_l1 = [(12,"D".to_string()), (12,"C".to_string()), (12,"S".to_string()), (12,"H".to_string()),(9,"C".to_string())];
	let mut rank3_w1 = [(13,"D".to_string()), (13,"C".to_string()), (13,"S".to_string()), (13,"H".to_string()),(9,"C".to_string())];
	
	let mut rank3_w2 = [(13,"D".to_string()), (13,"C".to_string()), (13,"S".to_string()), (13,"H".to_string()),(9,"C".to_string())];
	let mut rank3_l2 = [(13,"D".to_string()), (13,"C".to_string()), (13,"S".to_string()), (13,"H".to_string()),(1,"C".to_string())];
	
	let mut rank4_l = [(10,"D".to_string()), (10,"C".to_string()), (10,"S".to_string()), (12,"H".to_string()),(12,"C".to_string())];
	let mut rank4_w = [(12,"D".to_string()), (12,"C".to_string()), (12,"S".to_string()), (10,"H".to_string()),(10,"C".to_string())];
	
	let mut rank5_w = [(10,"D".to_string()), (2,"D".to_string()), (14,"D".to_string()), (11,"D".to_string()),(4,"D".to_string())];
	let mut rank5_l = [(8,"D".to_string()), (2,"D".to_string()), (14,"D".to_string()), (11,"D".to_string()),(4,"D".to_string())];
	
	let mut rank6_w = [(8,"D".to_string()), (9,"C".to_string()), (10,"H".to_string()), (11,"D".to_string()),(12,"C".to_string())];
	let mut rank6_l = [(5,"D".to_string()), (6,"C".to_string()), (7,"H".to_string()), (8,"D".to_string()),(9,"C".to_string())];
	
	let mut rank7_w1 = [(7,"D".to_string()), (7,"C".to_string()), (7,"S".to_string()), (13,"H".to_string()),(3,"C".to_string())];
	let mut rank7_l1 = [(4,"D".to_string()), (4,"C".to_string()), (4,"S".to_string()), (13,"H".to_string()),(3,"C".to_string())];
	
	let mut rank7_w2 = [(4,"D".to_string()), (4,"C".to_string()), (4,"S".to_string()), (13,"H".to_string()),(3,"C".to_string())];
	let mut rank7_l2 = [(4,"D".to_string()), (4,"C".to_string()), (4,"S".to_string()), (13,"H".to_string()),(2,"D".to_string())];
	
	let mut rank8_w1 = [(4,"D".to_string()), (4,"C".to_string()), (13,"S".to_string()), (13,"H".to_string()),(12,"C".to_string())];
	let mut rank8_l1 = [(4,"D".to_string()), (4,"C".to_string()), (3,"S".to_string()), (3,"H".to_string()),(12,"C".to_string())];

	let mut rank8_w2 = [(4,"D".to_string()), (4,"C".to_string()), (3,"S".to_string()), (3,"H".to_string()),(12,"C".to_string())];
	let mut rank8_l2 = [(4,"D".to_string()), (4,"C".to_string()), (3,"S".to_string()), (3,"H".to_string()),(10,"H".to_string())];
	
	let mut rank9_w1 = [(14,"D".to_string()), (14,"H".to_string()), (8,"C".to_string()), (4,"S".to_string()),(7,"H".to_string())];
	let mut rank9_l1= [(13,"D".to_string()), (13,"H".to_string()), (8,"C".to_string()), (4,"S".to_string()),(7,"H".to_string())];
	
	let mut rank9_w2 = [(14,"D".to_string()), (14,"H".to_string()), (8,"C".to_string()), (4,"S".to_string()),(7,"H".to_string())];
	let mut rank9_l2 = [(14,"D".to_string()), (14,"H".to_string()), (8,"C".to_string()), (2,"S".to_string()),(7,"H".to_string())];
	
	let mut rank10_w = [(3,"D".to_string()), (11,"C".to_string()), (8,"S".to_string()), (4,"H".to_string()),(2,"C".to_string())];
	let mut rank10_l = [(3,"D".to_string()), (11,"C".to_string()), (8,"S".to_string()), (4,"H".to_string()),(1,"C".to_string())];
	
	// test rank 1 against rank 2 winner
	println!("\n test rank 1 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank1_w, &mut rank2_w));
	// test rank 1 against rank 6 winner
	println!("\n test rank 1 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank1_w, &mut rank6_w));
	// test rank 2_w against rank 2_l winner
	println!("\n test rank 2 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank2_w, &mut rank2_l));
	// test rank 3_w1 against rank 3_l1 winner
	println!("\n test rank 3 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank3_w1, &mut rank3_l1));
	// test rank 3_w2 against rank 3_l2 winner
	println!("\n test rank 3 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank3_w2, &mut rank3_l2));
	// test rank 4_w against rank 4_l winner
	println!("\n test rank 4 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank4_w, &mut rank4_l));
	// test rank 5_w against rank 5_l winner
	println!("\n test rank 5 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank5_w, &mut rank5_l));
	// test rank 6_w against rank 6_l winner
	println!("\n test rank 6 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank6_w, &mut rank6_l));
	// test rank 7_w1 against rank 7_l1 winner
	println!("\n test rank 7 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank7_w1, &mut rank7_l1));
	// test rank 7_w2 against rank 7_l2 winner
	println!("\n test rank 7 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank7_w2, &mut rank7_l2));
	// test rank 8_w1 against rank 8_l1 winner
	println!("\n test rank 8 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank8_w1, &mut rank8_l1));
	// test rank 8_w2 against rank 8_l2 winner
	println!("\n test rank 8 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank8_w2, &mut rank8_l2));
	// test rank 9_w1 against rank 9_l1 winner
	println!("\n test rank 9 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank9_w1, &mut rank9_l1));
	// test rank 9_w2 against rank 9_l2 winner
	println!("\n test rank 9 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank9_w2, &mut rank9_l2));
	// test rank 10_w against rank 10_l winner
	println!("\n test rank 10 - > hand1 must win against hand2\nacntual result:\nhand{} is the winner", winner(&mut rank10_w, &mut rank10_l));
}
/* 
** 1 if hand1 is the winner, 2 if hand2 is the winner.
** if identical default winner is hand 1.
*/
fn winner(hand1: &mut [(usize, String)], 
		hand2:  &mut [(usize, String)]) -> u32
{
	let sorted_1 = sort_hand(hand1);
	let sorted_2 = sort_hand(hand2);
	let sign_1 = build_signature(sorted_1);
	let sign_2 = build_signature(sorted_2);
	let rank_1 = rank_signature(sign_1);
	let rank_2 = rank_signature(sign_2);
	/* ------------------------------
	** different ranks
	** ------------------------------
	*/
	if rank_1.0 < rank_2.0 { return 1 } 
	else if rank_2.0 < rank_1.0 { return 2 } // done with different ranks
	/* ------------------------------
	** similar rank cases, if identical default winner is hand 1
	** ------------------------------
	*/
	if rank_1.0 == 2 { 
		if sign_1.4 >= sign_2.4 { return 1 }
		else { return 2 }
	} // done with two Straight flush (2) hands
	
	if rank_1.0 == 3 { 
		if rank_1.1 > rank_2.1 { return 1 }
		else if rank_1.1 < rank_2.1 { return 2 }
		// down to highest card
		else if sorted_1[sorted_1.len() - 1] > sorted_2[sorted_2.len() - 1] {
			return 1
		}
		// down to the 5th card
		else if sorted_1[0] >= sorted_2[0] { return 1 }
		else { return 2 }
	} // done with two Four of a kind (3) hands
	
	if rank_1.0 == 4 || rank_1.0 == 7 || rank_1.0 == 8 || rank_1.0 == 9 {
		if (rank_1.1 + rank_1.2) > (rank_2.1 + rank_2.2) { return 1 }
		else if (rank_1.1 + rank_1.2) < (rank_2.1 + rank_2.2) { return 2 }
		// down to highest card (ranks 7, 8 or 9)
		let mut i1_max_rank = sorted_1.len() - 1;
		let mut i2_max_rank = sorted_2.len() - 1;
		// iterate through sorted list until the two are not the
		// same or list terminates
		while sorted_1[i1_max_rank] == sorted_2[i2_max_rank] &&
					i1_max_rank > 0 {
			i1_max_rank -= 1;
			i2_max_rank -= 1;
		}
		if sorted_1[i1_max_rank] >= sorted_2[i2_max_rank] { return 1 }
		else { return 2 }
	} // done with ranks 4, 7, 8 and 9
	
	if rank_1.0 == 5 || rank_1.0 == 10{
		let mut i1_max_rank = sorted_1.len() - 1;
		let mut i2_max_rank = sorted_2.len() - 1;
		// iterate through sorted list until the two are not the
		// same or list terminates
		while sorted_1[i1_max_rank] == sorted_2[i2_max_rank] &&
					i1_max_rank > 0 {
			i1_max_rank -= 1;
			i2_max_rank -= 1;
		}
		if sorted_1[i1_max_rank] >= sorted_2[i2_max_rank] { return 1 }
		else { return 2 }
	} // done with two Flush (5) and High Card (10) hands
	
	else {
		if sorted_1[sorted_1.len() - 1] >= 
				sorted_2[sorted_2.len() - 1] { return 1 }
		else { return 2 }
	} // done with two Straight (6) hands
}

/*
** Sort hand stably based on rank only from low to high. O(n^2)
** E.g. of a hand: (9, "H") for 9 of Heart.
*/
fn sort_hand(hand: &mut [(usize, String)]) -> &[(usize, String)]
{
	for i in 0..hand.len() {
		let mut jmin = i;
		for j in i+1..hand.len() {
			if hand[j].0 < hand[jmin].0 { // swap
				jmin = j;
			}
		}
		hand.swap(i, jmin);
	}
	hand
}

/*
** Create signature of a sorted hand (low->high) in following tuple format:
** (# of suits, same kinds1, same kinds2, consequative cards, highest rank,
		kinds1_strength, kinds2_strength)
** for instance Royal Flush has the following signature:
** (1, 0, 0, 4 *because all 5 in order(first card does not count)*, 
** 		14 *because ace*, 0, 0)
** or, a Full House will have the following signature:
** ( x >= 3 *depending*, *2, 1* or *1, 2*, y <= 1  *depending*, *any*, *any*,
**		*any'*)
** note: a "pair" results in kinds = 1 (not 2), and 3 of a kind is 
** kinds = 2 (not 3), etc.
*/
fn build_signature(sorted_hand: &[(usize, String)]) ->
	(usize, usize, usize, usize, usize, usize, usize) 
{
	// set holding unique 'chars'
	let suits; let mut suits_set = HashSet::new(); 
	// first set of same kinds, kind1over = 1 > done with set1, pair1 strength
	let mut kinds1 = 0; let mut kind1over = 0; let mut pair1st = 0;
	// second set of same kinds, pair2 strength
	let mut	kinds2 = 0; let mut pair2st = 0; 
	// consequetive cards where maximum is 4 for 5 cards in conseq order
	let mut conseqs = 0; let mut broke = 0;
	let max_rank = sorted_hand[4].0; // 1. get maximum rank in hand. Done
	
	// visit the hand only once to find the 5 elements of the signature
	for i in 1..sorted_hand.len(){
		let prev_tup = &sorted_hand[i-1]; // previous tuple
		let tup = &sorted_hand[i]; // current tuple
		// 2. to calculate #of suites
		suits_set.insert(&tup.1); // not done with 1. yet
		
		/* 3. find the sequence in one go only (e.g. hand
		** with ranks (1,2,3,11,12) results in conseq = 2 
		**(maximum of the two possible answers) rather 
		** than conseq = 2+1 or conseq = 1
		*/
		if prev_tup.0 == tup.0 - 1 && broke == 0 {// typical case
			conseqs += 1;
		}
		// broke once but this time more may be in sequence 
		else if prev_tup.0 == tup.0 - 1 && broke != 0 && conseqs == 1 {
			conseqs = 1; // since this happens once at max, just reset conseqs
		}
		else if conseqs != 0 && prev_tup.0 != tup.0 - 1 { // sequence breaks
			broke = 1;
		} // done with 3.
		
		/* 4. and 5. find all the sets of the same kinds.
		** will break if the sequence for "kind1" ends. Will 
		** then search for the second set in case exists.
		*/
		if prev_tup.0 == tup.0 && kind1over == 0 { // kind1 seq
			kinds1 += 1;
			pair1st = prev_tup.0;
		}
		else if prev_tup.0 != tup.0 && kinds1 != 0 { // seq. breaks
			kind1over = 1;
		}
		else if prev_tup.0 == tup.0 && kind1over == 1 { // kind2 seq
			kinds2 += 1;
			pair2st = prev_tup.0; 
		} // done with 4. and 5.
	}
	// (back to) 2. had skipped the first element
	suits_set.insert(&sorted_hand[0].1); 
	suits = suits_set.len(); // done with 2.
	// the signature (suits, kinds1, kinds2, conseqs, max_rank)
	let report = (suits, kinds1, kinds2, conseqs, max_rank, pair1st, pair2st);
	report
}

/*
** Rank Signature based on the details implied by the signature. To see 
** the structure of the signature, refer to the builder's method docs.
** Method assumes 'sensible' inputs only (see assignment).
** Also note that the rank strengths are multiplied by the relevant number of kinds
** to facilitate the comparison between two hands of the same ranks such as
** [(10, "D"), (10, "C"), (10, "S"), (12, "H"), (12, "C")] 
** [(12, "D"), (12, "C"), (12, "S"), (10, "H"), (10, "C")]
** both with signature = (4, 1, 2, 0, 12, 10, 12) have two different rank tuples:
** respectively: rank = (4, 10, 24) *total strength 24 + 10* and 
** rank = (4, 20, 12) *total strength 20 + 12*
** so the reason for the multiplication is so that the second hand wins over the first
** hand when the stregnth1 of the hand 1 is compared with stregnth1 of hand 2 and so on
** for strength 2. This makes up for the lost information when signature is passed to 
** another function, and for the rest of the cases this will not affect the results.
*/
fn rank_signature(s: (usize, usize, usize, usize, usize, usize, usize)) ->
	(usize, usize, usize)
{
	// recall the signature structure:
	// (#suits, same kind1, same kind2, conseq. cards, max r, pair1st, pair2st)
	let mut r = (10, 0, 0); // if none of below, 10 is the assumed rank rest N/A
	if s.0 == 1 && s.1 == 0 && s.2 == 0  {
		if s.3 == 4 && s.4 == 14 { r.0 = 1;} // royal
		else if s.3 == 4 { r.0 = 2; } // straight
		else { r.0 = 5; } // simply flush
	} // done with rank 1, 2 and 5
	else if s.0 == 4 && ((s.1 == 3 && s.2 == 0) || (s.1 == 0 && s.2 == 3))
			&& s.3 <= 1 {
		r.0 = 3;
		r.1 = s.5 * s.1;
		r.2 =s.6 * s.2;
	}// done with rank 3
	else if s.0 >= 3 && ((s.1 == 2 && s.2 == 1) || (s.1 == 1 && s.2 == 2))
			&&  s.3 <= 1 {
		r.0 = 4;
		r.1 = s.5 * s.1;
		r.2 =s.6 * s.2;
	}// done with rank 4
	else if s.3 == 4 {
		r.0 = 6;
	}// done with rank 6
	else if s.0 >= 3 && ((s.1 == 2 && s.2 == 0) || (s.1 == 0 && s.2 == 2))
			&& s.3 <= 2 {
		r.0 = 7;
		r.1 = s.5 * s.1;
		r.2 =s.6 * s.2;
	}// done with rank 7
	else if s.0 >= 2 {
		if s.1 == 1 && s.2 == 1 && s.3 <= 1 { r.0 = 8; r.1 = s.5; r.2 =s.6; }
		else if ((s.1 == 1 && s.2 == 0) || (s.1 == 0 && s.2 == 1))
			&& s.3 <= 3 { r.0 = 9; r.1 = s.5 * s.1; r.2 =s.6 * s.2; }
	} // done with rank 8 and 9
	r // reminder: rank = 10 if none of the above cond
}
