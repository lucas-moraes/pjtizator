use leptos::*;

struct CompanyFees {
    inss_company: f64,
    fgts: f64,
    rat: f64,
    salary_education: f64,
    s_system: f64,
    sum: f64,
    vacancy_fund_proportional: f64,
    salary_13_fund_proportional: f64,
}

fn format_currency(value: f64) -> String {
    let int_value = (value * 100.0).round() as i64;
    let formatted = format!("{}.{:02}", int_value / 100, int_value % 100);
    format!("R$ {}", formatted.replace(".", ","))
}

#[component]
pub fn Clt(
    salary: ReadSignal<String>,
    vale: ReadSignal<String>,
    deduct_clt: ReadSignal<String>,
    plr: ReadSignal<String>,
) -> impl IntoView {
    let calc_inss = move |salary: f64| -> f64 {
        let mut inss = 0.0;
        if salary > 0.0 && salary <= 1412.00 {
            inss = salary * 0.075;
        } else if salary > 1412.00 && salary <= 2666.68 {
            inss = 1412.00 * 0.075 + ((salary - 1412.00) * 0.09);
        } else if salary > 2666.68 && salary <= 4000.03 {
            inss = 1412.00 * 0.075 + ((2666.68 - 1412.00) * 0.09) + ((salary - 2666.68) * 0.12);
        } else if salary > 4000.03 && salary <= 7786.02 {
            inss = 1412.00 * 0.075
                + ((2666.68 - 1412.00) * 0.09)
                + ((4000.03 - 2666.68) * 0.12)
                + ((salary - 4000.03) * 0.14);
        } else if salary > 7786.02 {
            inss = 1412.00 * 0.075
                + ((2666.68 - 1412.00) * 0.09)
                + ((4000.03 - 2666.68) * 0.12)
                + ((7786.02 - 4000.03) * 0.14);
        }
        inss
    };

    let calc_irrf = move |salary_after_inss: f64| -> f64 {
        let mut irrf = 0.0;
        if salary_after_inss <= 2259.20 {
            irrf = 0.0;
        } else if salary_after_inss > 2259.20 && salary_after_inss <= 2826.65 {
            irrf = salary_after_inss * 0.075 - 169.44;
        } else if salary_after_inss > 2826.65 && salary_after_inss <= 3751.05 {
            irrf = salary_after_inss * 0.15 - 381.44;
        } else if salary_after_inss > 3751.05 && salary_after_inss <= 4664.68 {
            irrf = salary_after_inss * 0.225 - 662.77;
        } else if salary_after_inss > 4664.68 {
            irrf = salary_after_inss * 0.275 - 896.00;
        }
        irrf
    };

    let calc_net_salary = move || -> f64 {
        let salary_value = salary
            .get()
            .replace("R$ ", "")
            .replace(".", "")
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0);
        let vale_value = vale
            .get()
            .replace("R$ ", "")
            .replace(".", "")
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0);
        let deduct_value = deduct_clt
            .get()
            .replace("R$ ", "")
            .replace(".", "")
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0);
        let plr_value = plr
            .get()
            .replace("R$ ", "")
            .replace(".", "")
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0);
        let inss = calc_inss(salary_value);
        let salary_after_inss = salary_value - inss;
        let irrf = calc_irrf(salary_after_inss);
        let salary_after_irrf = salary_after_inss - irrf;
        let net = salary_after_irrf + vale_value - deduct_value + (plr_value / 12.0);
        return net;
    };

    let calc_company_fees = move || -> CompanyFees {
        let salary_value = salary
            .get()
            .replace("R$ ", "")
            .replace(".", "")
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0);
        let vacancy_fund_proportional_one = salary_value / 12.00;
        let vacancy_13_fund_proportional_two = vacancy_fund_proportional_one / 12.00 * 1.33;
        let vacancy_fund_proportional =
            vacancy_fund_proportional_one + vacancy_13_fund_proportional_two;
        let salary_13_fund_proportional = salary_value / 12.00;
        let inss_company = salary_value * 0.2;
        let fgts = salary_value * 0.08;
        let rat = salary_value * 0.01;
        let salary_education = salary_value * 0.025;
        let s_system = salary_value * 0.058;
        let sum = vacancy_fund_proportional + salary_13_fund_proportional + fgts;
        CompanyFees {
            inss_company,
            fgts,
            rat,
            salary_education,
            s_system,
            vacancy_fund_proportional,
            salary_13_fund_proportional,
            sum,
        }
    };

    view! {
        <div class="uk-marging-small  uk-padding-small  uk-card uk-card-default uk-card-body">
            <span class="uk-card-title uk-text-secondary">
                "CLT"
            </span>
            <hr class="uk-divider-icon" />
            <p>
                "Dados que aparecem na folha de pagamento:"
            </p>
            <ul class="uk-list uk-list-hyphen">
                <li class="uk-text-primary">
                    {move || format!("Salário bruto: {}", format_currency(salary.get().replace("R$ ", "").replace(".", "").replace(",", ".").trim().parse().unwrap_or(0.0)))}
                </li>
                <li class="uk-text-danger">
                    {move || format!("Desconto do INSS: {}", format_currency(calc_inss(salary.get().replace("R$ ", "").replace(".", "").replace(",", ".").trim().parse().unwrap_or(0.0))))}
                </li>
                <li class="uk-text-danger">
                    {move || format!("Desconto do IRRF: {}", {
                        let salary_value = salary.get().replace("R$ ", "").replace(".", "").replace(",", ".").trim().parse().unwrap_or(0.0);
                        let inss = calc_inss(salary_value);
                        format_currency(calc_irrf(salary_value - inss))
                    })}
                </li>
                <li class="uk-text-danger">
                    {move || format!("Descontos do diversos: {}", format_currency(deduct_clt.get().replace("R$ ", "").replace(".", "").replace(",", ".").trim().parse().unwrap_or(0.0)))}
                </li>
                <li class="uk-text-primary">
                    {move || format!("PLR divido por 12: {}", format_currency(plr.get().replace("R$ ", "").replace(".", "").replace(",", ".").trim().parse().unwrap_or(0.0) / 12.0))}
                </li>
                <li class="uk-text-primary">
                    {move || format!("Salário liquido: {}", format_currency(calc_net_salary()))}
                </li>
            </ul>
            <hr class="uk-divider-small" />
            <p>
                "Dados que não aparecem na folha de pagamento:"
            </p>
            <ul class="uk-list uk-list-hyphen">
                <li class="uk-text-primary">
                    {move || format!("Férias + 1/3 proporcional: {}", format_currency(calc_company_fees().vacancy_fund_proportional))}
                </li>
                <li class="uk-text-primary">
                    {move || format!("13º salário proporcional: {}", format_currency(calc_company_fees().salary_13_fund_proportional))}
                </li>
                <li class="uk-text-primary">
                    {move || format!("FGTS: {}", format_currency(calc_company_fees().fgts))}
                </li>
                <li class="uk-text-danger">
                    {move || format!("INSS da empresa: {}", format_currency(calc_company_fees().inss_company))}
                </li>
                <li class="uk-text-danger">
                    {move || format!("RAT: {}", format_currency(calc_company_fees().rat))}
                </li>
                <li class="uk-text-danger">
                    {move || format!("Salário educação: {}", format_currency(calc_company_fees().salary_education))}
                </li>
                <li class="uk-text-danger">
                    {move || format!("Sistema S: {}", format_currency(calc_company_fees().s_system))}
                </li>
            </ul>
             <hr class="uk-divider-icon" />
            <p class="uk-text-bolder">
            {move || format!("Remuneração geral liquida: {}", {
                let salary_liq = calc_net_salary();
                let company_fees = calc_company_fees();
                format_currency(salary_liq + company_fees.sum)
            })}
            </p>
        </div>
    }
}
