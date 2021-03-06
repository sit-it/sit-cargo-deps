#![no_std]
extern crate blake2;
#[macro_use]
extern crate crypto_mac;

use blake2::{Blake2b, Blake2s};
use crypto_mac::dev::{mac_test, Test};

#[test]
fn blake2b_mac() {
    let tests = new_mac_tests!(
        "blake2b/mac",
        "blake2b/mac/0",
        "blake2b/mac/1",
        "blake2b/mac/2",
        "blake2b/mac/3",
        "blake2b/mac/4",
        "blake2b/mac/5",
        "blake2b/mac/6",
        "blake2b/mac/7",
        "blake2b/mac/8",
        "blake2b/mac/9",
        "blake2b/mac/10",
        "blake2b/mac/11",
        "blake2b/mac/12",
        "blake2b/mac/13",
        "blake2b/mac/14",
        "blake2b/mac/15",
        "blake2b/mac/16",
        "blake2b/mac/17",
        "blake2b/mac/18",
        "blake2b/mac/19",
        "blake2b/mac/20",
        "blake2b/mac/21",
        "blake2b/mac/22",
        "blake2b/mac/23",
        "blake2b/mac/24",
        "blake2b/mac/25",
        "blake2b/mac/26",
        "blake2b/mac/27",
        "blake2b/mac/28",
        "blake2b/mac/29",
        "blake2b/mac/30",
        "blake2b/mac/31",
        "blake2b/mac/32",
        "blake2b/mac/33",
        "blake2b/mac/34",
        "blake2b/mac/35",
        "blake2b/mac/36",
        "blake2b/mac/37",
        "blake2b/mac/38",
        "blake2b/mac/39",
        "blake2b/mac/40",
        "blake2b/mac/41",
        "blake2b/mac/42",
        "blake2b/mac/43",
        "blake2b/mac/44",
        "blake2b/mac/45",
        "blake2b/mac/46",
        "blake2b/mac/47",
        "blake2b/mac/48",
        "blake2b/mac/49",
        "blake2b/mac/50",
        "blake2b/mac/51",
        "blake2b/mac/52",
        "blake2b/mac/53",
        "blake2b/mac/54",
        "blake2b/mac/55",
        "blake2b/mac/56",
        "blake2b/mac/57",
        "blake2b/mac/58",
        "blake2b/mac/59",
        "blake2b/mac/60",
        "blake2b/mac/61",
        "blake2b/mac/62",
        "blake2b/mac/63",
        "blake2b/mac/64",
        "blake2b/mac/65",
        "blake2b/mac/66",
        "blake2b/mac/67",
        "blake2b/mac/68",
        "blake2b/mac/69",
        "blake2b/mac/70",
        "blake2b/mac/71",
        "blake2b/mac/72",
        "blake2b/mac/73",
        "blake2b/mac/74",
        "blake2b/mac/75",
        "blake2b/mac/76",
        "blake2b/mac/77",
        "blake2b/mac/78",
        "blake2b/mac/79",
        "blake2b/mac/80",
        "blake2b/mac/81",
        "blake2b/mac/82",
        "blake2b/mac/83",
        "blake2b/mac/84",
        "blake2b/mac/85",
        "blake2b/mac/86",
        "blake2b/mac/87",
        "blake2b/mac/88",
        "blake2b/mac/89",
        "blake2b/mac/90",
        "blake2b/mac/91",
        "blake2b/mac/92",
        "blake2b/mac/93",
        "blake2b/mac/94",
        "blake2b/mac/95",
        "blake2b/mac/96",
        "blake2b/mac/97",
        "blake2b/mac/98",
        "blake2b/mac/99",
        "blake2b/mac/100",
        "blake2b/mac/101",
        "blake2b/mac/102",
        "blake2b/mac/103",
        "blake2b/mac/104",
        "blake2b/mac/105",
        "blake2b/mac/106",
        "blake2b/mac/107",
        "blake2b/mac/108",
        "blake2b/mac/109",
        "blake2b/mac/110",
        "blake2b/mac/111",
        "blake2b/mac/112",
        "blake2b/mac/113",
        "blake2b/mac/114",
        "blake2b/mac/115",
        "blake2b/mac/116",
        "blake2b/mac/117",
        "blake2b/mac/118",
        "blake2b/mac/119",
        "blake2b/mac/120",
        "blake2b/mac/121",
        "blake2b/mac/122",
        "blake2b/mac/123",
        "blake2b/mac/124",
        "blake2b/mac/125",
        "blake2b/mac/126",
        "blake2b/mac/127",
        "blake2b/mac/128",
        "blake2b/mac/129",
        "blake2b/mac/130",
        "blake2b/mac/131",
        "blake2b/mac/132",
        "blake2b/mac/133",
        "blake2b/mac/134",
        "blake2b/mac/135",
        "blake2b/mac/136",
        "blake2b/mac/137",
        "blake2b/mac/138",
        "blake2b/mac/139",
        "blake2b/mac/140",
        "blake2b/mac/141",
        "blake2b/mac/142",
        "blake2b/mac/143",
        "blake2b/mac/144",
        "blake2b/mac/145",
        "blake2b/mac/146",
        "blake2b/mac/147",
        "blake2b/mac/148",
        "blake2b/mac/149",
        "blake2b/mac/150",
        "blake2b/mac/151",
        "blake2b/mac/152",
        "blake2b/mac/153",
        "blake2b/mac/154",
        "blake2b/mac/155",
        "blake2b/mac/156",
        "blake2b/mac/157",
        "blake2b/mac/158",
        "blake2b/mac/159",
        "blake2b/mac/160",
        "blake2b/mac/161",
        "blake2b/mac/162",
        "blake2b/mac/163",
        "blake2b/mac/164",
        "blake2b/mac/165",
        "blake2b/mac/166",
        "blake2b/mac/167",
        "blake2b/mac/168",
        "blake2b/mac/169",
        "blake2b/mac/170",
        "blake2b/mac/171",
        "blake2b/mac/172",
        "blake2b/mac/173",
        "blake2b/mac/174",
        "blake2b/mac/175",
        "blake2b/mac/176",
        "blake2b/mac/177",
        "blake2b/mac/178",
        "blake2b/mac/179",
        "blake2b/mac/180",
        "blake2b/mac/181",
        "blake2b/mac/182",
        "blake2b/mac/183",
        "blake2b/mac/184",
        "blake2b/mac/185",
        "blake2b/mac/186",
        "blake2b/mac/187",
        "blake2b/mac/188",
        "blake2b/mac/189",
        "blake2b/mac/190",
        "blake2b/mac/191",
        "blake2b/mac/192",
        "blake2b/mac/193",
        "blake2b/mac/194",
        "blake2b/mac/195",
        "blake2b/mac/196",
        "blake2b/mac/197",
        "blake2b/mac/198",
        "blake2b/mac/199",
        "blake2b/mac/200",
        "blake2b/mac/201",
        "blake2b/mac/202",
        "blake2b/mac/203",
        "blake2b/mac/204",
        "blake2b/mac/205",
        "blake2b/mac/206",
        "blake2b/mac/207",
        "blake2b/mac/208",
        "blake2b/mac/209",
        "blake2b/mac/210",
        "blake2b/mac/211",
        "blake2b/mac/212",
        "blake2b/mac/213",
        "blake2b/mac/214",
        "blake2b/mac/215",
        "blake2b/mac/216",
        "blake2b/mac/217",
        "blake2b/mac/218",
        "blake2b/mac/219",
        "blake2b/mac/220",
        "blake2b/mac/221",
        "blake2b/mac/222",
        "blake2b/mac/223",
        "blake2b/mac/224",
        "blake2b/mac/225",
        "blake2b/mac/226",
        "blake2b/mac/227",
        "blake2b/mac/228",
        "blake2b/mac/229",
        "blake2b/mac/230",
        "blake2b/mac/231",
        "blake2b/mac/232",
        "blake2b/mac/233",
        "blake2b/mac/234",
        "blake2b/mac/235",
        "blake2b/mac/236",
        "blake2b/mac/237",
        "blake2b/mac/238",
        "blake2b/mac/239",
        "blake2b/mac/240",
        "blake2b/mac/241",
        "blake2b/mac/242",
        "blake2b/mac/243",
        "blake2b/mac/244",
        "blake2b/mac/245",
        "blake2b/mac/246",
        "blake2b/mac/247",
        "blake2b/mac/248",
        "blake2b/mac/249",
        "blake2b/mac/250",
        "blake2b/mac/251",
        "blake2b/mac/252",
        "blake2b/mac/253",
        "blake2b/mac/254",
        "blake2b/mac/255"
    );
    mac_test::<Blake2b>(&tests);
}

#[test]
fn blake2s_mac() {
    let tests = new_mac_tests!(
        "blake2s/mac",
        "blake2s/mac/0",
        "blake2s/mac/1",
        "blake2s/mac/2",
        "blake2s/mac/3",
        "blake2s/mac/4",
        "blake2s/mac/5",
        "blake2s/mac/6",
        "blake2s/mac/7",
        "blake2s/mac/8",
        "blake2s/mac/9",
        "blake2s/mac/10",
        "blake2s/mac/11",
        "blake2s/mac/12",
        "blake2s/mac/13",
        "blake2s/mac/14",
        "blake2s/mac/15",
        "blake2s/mac/16",
        "blake2s/mac/17",
        "blake2s/mac/18",
        "blake2s/mac/19",
        "blake2s/mac/20",
        "blake2s/mac/21",
        "blake2s/mac/22",
        "blake2s/mac/23",
        "blake2s/mac/24",
        "blake2s/mac/25",
        "blake2s/mac/26",
        "blake2s/mac/27",
        "blake2s/mac/28",
        "blake2s/mac/29",
        "blake2s/mac/30",
        "blake2s/mac/31",
        "blake2s/mac/32",
        "blake2s/mac/33",
        "blake2s/mac/34",
        "blake2s/mac/35",
        "blake2s/mac/36",
        "blake2s/mac/37",
        "blake2s/mac/38",
        "blake2s/mac/39",
        "blake2s/mac/40",
        "blake2s/mac/41",
        "blake2s/mac/42",
        "blake2s/mac/43",
        "blake2s/mac/44",
        "blake2s/mac/45",
        "blake2s/mac/46",
        "blake2s/mac/47",
        "blake2s/mac/48",
        "blake2s/mac/49",
        "blake2s/mac/50",
        "blake2s/mac/51",
        "blake2s/mac/52",
        "blake2s/mac/53",
        "blake2s/mac/54",
        "blake2s/mac/55",
        "blake2s/mac/56",
        "blake2s/mac/57",
        "blake2s/mac/58",
        "blake2s/mac/59",
        "blake2s/mac/60",
        "blake2s/mac/61",
        "blake2s/mac/62",
        "blake2s/mac/63",
        "blake2s/mac/64",
        "blake2s/mac/65",
        "blake2s/mac/66",
        "blake2s/mac/67",
        "blake2s/mac/68",
        "blake2s/mac/69",
        "blake2s/mac/70",
        "blake2s/mac/71",
        "blake2s/mac/72",
        "blake2s/mac/73",
        "blake2s/mac/74",
        "blake2s/mac/75",
        "blake2s/mac/76",
        "blake2s/mac/77",
        "blake2s/mac/78",
        "blake2s/mac/79",
        "blake2s/mac/80",
        "blake2s/mac/81",
        "blake2s/mac/82",
        "blake2s/mac/83",
        "blake2s/mac/84",
        "blake2s/mac/85",
        "blake2s/mac/86",
        "blake2s/mac/87",
        "blake2s/mac/88",
        "blake2s/mac/89",
        "blake2s/mac/90",
        "blake2s/mac/91",
        "blake2s/mac/92",
        "blake2s/mac/93",
        "blake2s/mac/94",
        "blake2s/mac/95",
        "blake2s/mac/96",
        "blake2s/mac/97",
        "blake2s/mac/98",
        "blake2s/mac/99",
        "blake2s/mac/100",
        "blake2s/mac/101",
        "blake2s/mac/102",
        "blake2s/mac/103",
        "blake2s/mac/104",
        "blake2s/mac/105",
        "blake2s/mac/106",
        "blake2s/mac/107",
        "blake2s/mac/108",
        "blake2s/mac/109",
        "blake2s/mac/110",
        "blake2s/mac/111",
        "blake2s/mac/112",
        "blake2s/mac/113",
        "blake2s/mac/114",
        "blake2s/mac/115",
        "blake2s/mac/116",
        "blake2s/mac/117",
        "blake2s/mac/118",
        "blake2s/mac/119",
        "blake2s/mac/120",
        "blake2s/mac/121",
        "blake2s/mac/122",
        "blake2s/mac/123",
        "blake2s/mac/124",
        "blake2s/mac/125",
        "blake2s/mac/126",
        "blake2s/mac/127",
        "blake2s/mac/128",
        "blake2s/mac/129",
        "blake2s/mac/130",
        "blake2s/mac/131",
        "blake2s/mac/132",
        "blake2s/mac/133",
        "blake2s/mac/134",
        "blake2s/mac/135",
        "blake2s/mac/136",
        "blake2s/mac/137",
        "blake2s/mac/138",
        "blake2s/mac/139",
        "blake2s/mac/140",
        "blake2s/mac/141",
        "blake2s/mac/142",
        "blake2s/mac/143",
        "blake2s/mac/144",
        "blake2s/mac/145",
        "blake2s/mac/146",
        "blake2s/mac/147",
        "blake2s/mac/148",
        "blake2s/mac/149",
        "blake2s/mac/150",
        "blake2s/mac/151",
        "blake2s/mac/152",
        "blake2s/mac/153",
        "blake2s/mac/154",
        "blake2s/mac/155",
        "blake2s/mac/156",
        "blake2s/mac/157",
        "blake2s/mac/158",
        "blake2s/mac/159",
        "blake2s/mac/160",
        "blake2s/mac/161",
        "blake2s/mac/162",
        "blake2s/mac/163",
        "blake2s/mac/164",
        "blake2s/mac/165",
        "blake2s/mac/166",
        "blake2s/mac/167",
        "blake2s/mac/168",
        "blake2s/mac/169",
        "blake2s/mac/170",
        "blake2s/mac/171",
        "blake2s/mac/172",
        "blake2s/mac/173",
        "blake2s/mac/174",
        "blake2s/mac/175",
        "blake2s/mac/176",
        "blake2s/mac/177",
        "blake2s/mac/178",
        "blake2s/mac/179",
        "blake2s/mac/180",
        "blake2s/mac/181",
        "blake2s/mac/182",
        "blake2s/mac/183",
        "blake2s/mac/184",
        "blake2s/mac/185",
        "blake2s/mac/186",
        "blake2s/mac/187",
        "blake2s/mac/188",
        "blake2s/mac/189",
        "blake2s/mac/190",
        "blake2s/mac/191",
        "blake2s/mac/192",
        "blake2s/mac/193",
        "blake2s/mac/194",
        "blake2s/mac/195",
        "blake2s/mac/196",
        "blake2s/mac/197",
        "blake2s/mac/198",
        "blake2s/mac/199",
        "blake2s/mac/200",
        "blake2s/mac/201",
        "blake2s/mac/202",
        "blake2s/mac/203",
        "blake2s/mac/204",
        "blake2s/mac/205",
        "blake2s/mac/206",
        "blake2s/mac/207",
        "blake2s/mac/208",
        "blake2s/mac/209",
        "blake2s/mac/210",
        "blake2s/mac/211",
        "blake2s/mac/212",
        "blake2s/mac/213",
        "blake2s/mac/214",
        "blake2s/mac/215",
        "blake2s/mac/216",
        "blake2s/mac/217",
        "blake2s/mac/218",
        "blake2s/mac/219",
        "blake2s/mac/220",
        "blake2s/mac/221",
        "blake2s/mac/222",
        "blake2s/mac/223",
        "blake2s/mac/224",
        "blake2s/mac/225",
        "blake2s/mac/226",
        "blake2s/mac/227",
        "blake2s/mac/228",
        "blake2s/mac/229",
        "blake2s/mac/230",
        "blake2s/mac/231",
        "blake2s/mac/232",
        "blake2s/mac/233",
        "blake2s/mac/234",
        "blake2s/mac/235",
        "blake2s/mac/236",
        "blake2s/mac/237",
        "blake2s/mac/238",
        "blake2s/mac/239",
        "blake2s/mac/240",
        "blake2s/mac/241",
        "blake2s/mac/242",
        "blake2s/mac/243",
        "blake2s/mac/244",
        "blake2s/mac/245",
        "blake2s/mac/246",
        "blake2s/mac/247",
        "blake2s/mac/248",
        "blake2s/mac/249",
        "blake2s/mac/250",
        "blake2s/mac/251",
        "blake2s/mac/252",
        "blake2s/mac/253",
        "blake2s/mac/254",
        "blake2s/mac/255"
    );
    mac_test::<Blake2s>(&tests);
}

