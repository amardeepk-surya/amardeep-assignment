use crate::*;

#[derive(Debug)]
struct OutVal {
    emp_name: String,
    dept_title: String,
    mob_no: String,
    e_mail: String,
    sal_status: String,
    on_leave: i32,
}
#[derive(Debug)]
pub struct OutSt {
    emp_id: HashMap<i32, OutVal>,
}

impl OutSt {
    pub fn new(
        pth: &Pth,
        deptmp: &HashMap<i32, String>,
        salmp: &HashMap<i32, String>,
        lvmp: &HashMap<i32, i32>,
    ) -> Result<OutSt, std::io::Error> {
        let mut f = std::fs::OpenOptions::new().read(true).open(&pth.e)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let mut hshmp = HashMap::new();
        let mut i = 0;
        for line in content.lines() {
            if i == 0 {
                i = 1;
                continue;
            }
            let mut values = line.split('|');
            let empid = values.next().expect("No empid");
            let empname = values.next().expect("No empname");
            let deptid = values.next().expect("No deptid");
            let mobno = values.next().expect("No mobno");
            let email = values.next().expect("No email");
            let empidp = empid.parse::<i32>().unwrap();
            let depttitle = deptmp.get(&deptid.parse::<i32>().unwrap()).unwrap();
            let salstatus: String = match salmp.get(&empidp) {
                Some(v) => v.to_string(),
                _ => "Not Credited".to_string(),
            };
            let totlv: i32 = match lvmp.get(&empidp) {
                Some(v) => *v,
                _ => 0,
            };
            let outval = OutVal {
                emp_name: empname.to_string(),
                mob_no: mobno.to_string(),
                e_mail: email.to_string(),
                dept_title: depttitle.to_string(),
                on_leave: totlv,
                sal_status: salstatus,
            };
            hshmp.insert(empidp, outval);
        }
        Ok(OutSt { emp_id: hshmp })
    }

    pub fn save(self, pth: &Pth) -> Result<(), std::io::Error> {
        let mut content = String::new();
        let mut content = String::new();

        let emp_name_title = "Emp_name";
        let emp_id_title = "Emp_id";
        let dept_title_title = "dept_title";
        let mobile_no_title = "mobile_number";
        let email_title = "email";
        let salary_status_title = "salary_status";
        let on_leave_title = "on_leave";

        let record = format!(
            "{}#{}#{}#{}#{}#{}#{}\n",
            emp_id_title,
            emp_name_title,
            dept_title_title,
            mobile_no_title,
            email_title,
            salary_status_title,
            on_leave_title
        );
        content.push_str(&record);
        for (k, v) in self.emp_id {
            let record = format!(
                "{}#{}#{}#{}#{}#{}#{}\n",
                k, v.emp_name, v.dept_title, v.mob_no, v.e_mail, v.sal_status, v.on_leave
            );
            content.push_str(&record)
        }
        std::fs::write(&pth.o, content)
    }
}
