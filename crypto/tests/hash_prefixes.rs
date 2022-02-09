#[cfg(test)]
mod tests {
    use ark_ff::BigInteger256;
    use mina_crypto::hash::prefixes::*;
    use num_bigint::BigUint;

    #[test]
    fn create_works_as_expected() {
        assert_eq!(PROTOCOL_STATE.len(), 20);
        assert_eq!(PROTOCOL_STATE, b"CodaProtoState******");
    }

    #[test]
    fn make_merkle_tree_hash_3() {
        let prefix_at_3 = make_prefix_merkle_tree(3);
        assert_eq!(prefix_at_3.len(), 20);
        assert_eq!(&prefix_at_3, b"CodaMklTree003******");
    }

    #[test]
    fn make_merkle_tree_hash_13() {
        let prefix_at_3 = make_prefix_merkle_tree(13);
        assert_eq!(prefix_at_3.len(), 20);
        assert_eq!(&prefix_at_3, b"CodaMklTree013******");
    }

    #[test]
    fn make_merkle_tree_hash_113() {
        let prefix_at_3 = make_prefix_merkle_tree(113);
        assert_eq!(prefix_at_3.len(), 20);
        assert_eq!(&prefix_at_3, b"CodaMklTree113******");
    }

    #[test]
    fn make_coinbase_merkle_tree_hash() {
        let prefix_at_3 = make_prefix_coinbase_merkle_tree(3);
        assert_eq!(prefix_at_3.len(), 20);
        assert_eq!(&prefix_at_3, b"CodaCbMklTree003****");
    }

    // Test cases are generated from ocaml code
    // add inline ocaml code to any unittests in
    // <https://github.com/MinaProtocol/mina/blob/compatible/src/lib/random_oracle/random_oracle.ml>
    // run `dune test` under src/lib/random_oracle
    #[test]
    fn test_prefix_to_fields() {
        // Printf.printf "%s" ("" |> prefix_to_field |> Field.to_string) ;
        test_prefix_to_field(b"", "0");
        // Printf.printf "%s" ("1" |> prefix_to_field |> Field.to_string) ;
        test_prefix_to_field(b"1", "49");
        // Printf.printf "%s" ("12" |> prefix_to_field |> Field.to_string) ;
        test_prefix_to_field(b"12", "12849");
        // Printf.printf "%s" ("123" |> prefix_to_field |> Field.to_string) ;
        test_prefix_to_field(b"123", "3355185");
        // Printf.printf "%s" ("AbC" |> prefix_to_field |> Field.to_string) ;
        test_prefix_to_field(b"AbC", "4416065");
        // Printf.printf "%s" ("AbC" |> prefix_to_field |> Field.to_string) ;
        test_prefix_to_field(b"AbC", "4416065");
        // Printf.printf "%s" ("CodaMklTree003******" |> prefix_to_field |> Field.to_string) ;
        test_prefix_to_field(
            b"CodaMklTree003******",
            "240717916736854781311355544089949626038405590851",
        );
    }

    fn test_prefix_to_field(prefix: &[u8], expected_field_str: &str) {
        let f = prefix_to_field(prefix).unwrap();
        let big256: BigInteger256 = f.into();
        let big: BigUint = big256.into();
        assert_eq!(expected_field_str, big.to_str_radix(10))
    }

    // Test cases are generated from ocaml code
    // add below ocaml code to any unittests in
    // <https://github.com/MinaProtocol/mina/blob/compatible/src/lib/random_oracle/random_oracle.ml>
    // run `dune test` under src/lib/random_oracle
    //
    // let fields = "CodaMklTree003******" |> salt in
    //   for i = 0 to Array.length fields - 1 do
    //   Printf.printf "\"%s\",\n" (fields.(i) |> Field.to_string)
    // done ;
    //
    #[test]
    fn test_salt() {
        test_salt_inner(
            b"",
            &[
                "10810255668636942098026103766265049994195917059170783454356350086236922262043",
                "20369500102464710973032553963549505382488144090419421689712140028735622028327",
                "23199662569381889769462525393322679512135888284607235517285306434130713031505",
            ],
        );
        test_salt_inner(
            b"1",
            &[
                "26959872850589468433716176478758670364938559091018984464547761444407740430490",
                "2912877462946461802595362763538105930120188813887961425049622298476285287509",
                "25751709115320426463100541822106182596074955216799512201836609258257127172454",
            ],
        );
        test_salt_inner(
            b"12",
            &[
                "25413427419476335337191819532645296442286213094707480147794752856248042737641",
                "5318335908100741006501788377285327387134432144519405055607069146838738276298",
                "21851504850499680971383647984837601926602580239148151982886754639614483383049",
            ],
        );
        test_salt_inner(
            b"0",
            &[
                "9687080897238730679281332027647963754910144833562264548730594001666765199615",
                "14181546407370227502650496841647492436510039486471099016097868622271393086085",
                "442126543670066912644781665934079626630208057488650742218981350471105366026",
            ],
        );
        test_salt_inner(
            b"CodaMklTree003******",
            &[
                "4759486266429649359680583704294416062325271909975044439354030843449421998024",
                "20959467482874439523768445553897628827309247681867820797074898520853041807408",
                "23898473196000807214630503958354143234825691497475523006976384309715021272572",
            ],
        );
    }

    fn test_salt_inner(prefix: &[u8], expected_fields: &[&str]) {
        let fields = salt(prefix).unwrap();
        assert_eq!(fields.len(), expected_fields.len());
        for (i, f) in fields.into_iter().enumerate() {
            let big256: BigInteger256 = f.into();
            let big: BigUint = big256.into();
            assert_eq!(expected_fields[i], big.to_str_radix(10))
        }
    }

    // Test cases are generated from ocaml code
    // add below ocaml code to any unittests in
    // <https://github.com/MinaProtocol/mina/blob/compatible/src/lib/random_oracle/random_oracle.ml>
    // run `dune test` under src/lib/random_oracle
    //
    // let fields = "" |> salt in
    //   Printf.printf "%s\n" (hash ~init:fields fields |> Field.to_string) ;
    //
    #[test]
    fn test_hash() {
        test_hash_inner(
            b"",
            "24205185403244298026878196079710983462016752932337361617747822459876338913053",
        );
        test_hash_inner(
            b"0",
            "5496131356387564043198525479062691436049129693667187354714170366167796524973",
        );
        test_hash_inner(
            b"1",
            "9935913569175172664639067019488671388546456308890708045000229809977751864941",
        );
        test_hash_inner(
            b"12",
            "943243131985716223594928077527124171127078993271980814573823847313318392653",
        );
        test_hash_inner(
            b"CodaMklTree003******",
            "22053519500510895302504638888009899630643515378059175829722187193750278074165",
        );
    }

    fn test_hash_inner(prefix: &[u8], expected_field: &str) {
        let fields = salt(prefix).unwrap();
        let h = hash(fields.clone(), &fields);
        let big256: BigInteger256 = h.into();
        let big: BigUint = big256.into();
        assert_eq!(expected_field, big.to_str_radix(10))
    }
}
