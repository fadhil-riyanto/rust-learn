pub fn ownership2()
{
        let somenewstr: String = retownership();
        // takeownership(somenewstr);
        let b: String = giveback(somenewstr);
        println!("{}", b);

}

fn retownership() -> String
{
        let somestr: String = String::from("adasdas");
        return somestr;
}

fn takeownership(data: String)
{
        println!("{}", data);
}

fn giveback(data: String) -> String
{
        return data;
}