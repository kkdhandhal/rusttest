pub struct MstLdg{
        month:u8,
        year:u16,
        /*circle_code:u16,
        circle_name:String,
        div_code:u8,
        div_name:String,
        sdn_code:u16,
        sdn_name:String,
        loc_code:u32,
        cons_no:u64,
        cons_name:String,
        add1:String,
        add2:String,
        cencus_code:u64,
        tran_loc:String,
        sd_amt:f64,
        conn_date:String,
	    pole_code:String,
	    pdc_date:String,
	    ind_type:String,
	    seasonal_ind:String,
	    sd_amount_receipt_date:String,
	    sd_amount_receipt_no:String,
	    contract_load:f32,
	    tarrif:String,
	    cycle_no:u8,
	    meter_reader_no:u16,
	    book_no:u16,
	    route_code:u16,
	    meter_no:String,
	    meter_status:String,
	    eduty_code:u16,
	    eduty_amount:f32,
	    contract_demand:f32,
	    feeder_no:u8,
	    phase:String,
	    meter_rent_type:String,
	    theft_arrears:f64,
	    litigation_arrears:f64,
	    fixed_chg:f32,
	    fuel_chg:f32,
	    fuse_chg:f32,
	    credit_adjustment:f64,
	    debit_adjustment:f64,
	    closing_arrears:f64,
	    consumption_unit:i64,
	    assessment_unit:i64,
	    dpc_amount:f64,
	    relief_amount:f64,
	    energy_charge:f64,
	    board_charge:f64,
	    prov_bill_amount:f64,
	    payment_amount:f64,
	    bill_amount:f64,
	    start_meter:i64,
	    end_meter:i64,
	    avg_unit:u32,
	    re_start_reading:i64,
	    re_end_reading:i64,
	    re_consumption:i64,
	    bill_date:String,
	    due_date:String,
	    payment_date:String,
	    feeder_code:u32,
	    feeder_name:String,
	    village_name:String,
	    tarrif_short:String,
	    meter_change_date:String,
	    adjustment_unit:i64,
	    old_tariff_ind:String,
	    bill_demand:f32,
	    bill_type:f32,
	    status:String,
        last_paydate:String,
        prev_bill_date:String,*/

	}


fn parse_data(line:String) -> MstLdg{
	let token:Vec<String>= line.split("|").map(|s| s.to_string()).collect();
	let mut mstldg:MstLdg = MstLdg{
		month : match token[0].parse() {
			Ok(n) => n,
			Err(e) => {0},
		},
		//assert_eq!(mstldg.month.is_ascii_digit())
		year : match token[1].parse() {
			Ok(n) => n,
			Err(e) => {0},
		},
	}; 
	
	
	
	return  mstldg;
}
pub fn insert_data(line:String){
	//println!("{}",line);
	let mstldg1 = parse_data(line);
	println!("Month Is {} and year is {}", mstldg1.month,mstldg1.year);
}

 
