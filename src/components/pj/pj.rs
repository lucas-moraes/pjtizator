use leptos::*;

struct CurrencyCorporate {
    billing: f64,
    tax_payable: f64,
}

struct ProLaoreMinimum {
    minimum_salary: f64,
    inss_pro_labore: f64,
}

struct ProLaborePercent {
    pro_labore: f64,
    inss_pro_labore: f64,
    irrf_pro_labore: f64,
}

fn format_currency(value: f64) -> String {
    let int_value = (value * 100.0).round() as i64;
    let formatted = format!("{}.{:02}", int_value / 100, int_value % 100);
    format!("R$ {}", formatted.replace(".", ","))
}

#[component]
pub fn Pj(
    invoice: ReadSignal<String>,
    vale: ReadSignal<String>,
    deduct_pj: ReadSignal<String>,
) -> impl IntoView {
    let minimum_salary = 1412.00;

    let calc_anexo_III = move || -> CurrencyCorporate {
        let invoice = invoice
            .get()
            .replace("R$ ", "")
            .replace(".", "")
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0);
        let vale = vale
            .get()
            .replace("R$ ", "")
            .replace(".", "")
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0);
        let billing_monthly = invoice + vale;
        let invoice_12_months = billing_monthly * 12.0;
        let mut tax_payable = 0.0;
        let effective_tax;

        if invoice_12_months == 0.0 {
            return CurrencyCorporate {
                billing: 0.0,
                tax_payable: 0.0,
            };
        }

        if invoice_12_months <= 180000.0 {
            effective_tax = (invoice_12_months * 0.06) / invoice_12_months;
            tax_payable = billing_monthly * effective_tax;
        } else if invoice_12_months > 180000.0 && invoice_12_months <= 360000.0 {
            effective_tax = (invoice_12_months * 0.112 - 9360.0) / invoice_12_months;
            tax_payable = billing_monthly * effective_tax;
        } else if invoice_12_months > 360000.0 && invoice_12_months <= 720000.0 {
            effective_tax = (invoice_12_months * 0.135 - 17640.0) / invoice_12_months;
            tax_payable = billing_monthly * effective_tax;
        } else if invoice_12_months > 720000.0 && invoice_12_months <= 1800000.0 {
            effective_tax = (invoice_12_months * 0.16 - 35640.0) / invoice_12_months;
            tax_payable = billing_monthly * effective_tax;
        } else if invoice_12_months > 1800000.0 && invoice_12_months <= 3600000.0 {
            effective_tax = (invoice_12_months * 0.21 - 125640.0) / invoice_12_months;
            tax_payable = billing_monthly * effective_tax;
        } else if invoice_12_months > 3600000.0 {
            effective_tax = (invoice_12_months * 0.33 - 648000.0) / invoice_12_months;
            tax_payable = billing_monthly * effective_tax;
        }

        CurrencyCorporate {
            billing: billing_monthly,
            tax_payable,
        }
    };

    let calc_anexo_V = move || -> CurrencyCorporate {
        let invoice = invoice
            .get()
            .replace("R$ ", "")
            .replace(".", "")
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0);
        let vale = vale
            .get()
            .replace("R$ ", "")
            .replace(".", "")
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0);
        let billing_monthly = invoice + vale;
        let invoice_12_months = billing_monthly * 12.0;
        let mut tax_payable = 0.0;
        let effective_tax;

        if invoice_12_months == 0.0 {
            return CurrencyCorporate {
                billing: 0.0,
                tax_payable: 0.0,
            };
        }

        if invoice_12_months <= 180000.0 {
            effective_tax = (invoice_12_months * 0.155) / invoice_12_months;
            tax_payable = billing_monthly * effective_tax;
        } else if invoice_12_months > 180000.0 && invoice_12_months <= 360000.0 {
            effective_tax = (invoice_12_months * 0.18 - 4500.0) / invoice_12_months;
            tax_payable = billing_monthly * effective_tax;
        } else if invoice_12_months > 360000.0 && invoice_12_months <= 720000.0 {
            effective_tax = (invoice_12_months * 0.195 - 9900.0) / invoice_12_months;
            tax_payable = billing_monthly * effective_tax;
        } else if invoice_12_months > 720000.0 && invoice_12_months <= 1800000.0 {
            effective_tax = (invoice_12_months * 0.205 - 17100.0) / invoice_12_months;
            tax_payable = billing_monthly * effective_tax;
        } else if invoice_12_months > 1800000.0 && invoice_12_months <= 3600000.0 {
            effective_tax = (invoice_12_months * 0.23 - 62100.0) / invoice_12_months;
            tax_payable = billing_monthly * effective_tax;
        } else if invoice_12_months > 3600000.0 {
            effective_tax = (invoice_12_months * 0.305 - 540000.0) / invoice_12_months;
            tax_payable = billing_monthly * effective_tax;
        }

        CurrencyCorporate {
            billing: billing_monthly,
            tax_payable,
        }
    };

    let pro_labore_minimum = move || -> ProLaoreMinimum {
        let inss_pro_labore = minimum_salary * 0.11;

        if !invoice.get().is_empty() {
            ProLaoreMinimum {
                minimum_salary,
                inss_pro_labore,
            }
        } else {
            ProLaoreMinimum {
                minimum_salary: 0.0,
                inss_pro_labore: 0.0,
            }
        }
    };

    let calc_irrf = move |salary_after_inss: f64| -> f64 {
        // Tabela progressiva IRRF 2026
        let mut imposto = 0.0;
        if salary_after_inss <= 2428.80 {
            imposto = 0.0;
        } else if salary_after_inss > 2428.80 && salary_after_inss <= 2826.65 {
            imposto = salary_after_inss * 0.075 - 182.16;
        } else if salary_after_inss > 2826.65 && salary_after_inss <= 3751.05 {
            imposto = salary_after_inss * 0.15 - 394.16;
        } else if salary_after_inss > 3751.05 && salary_after_inss <= 4664.68 {
            imposto = salary_after_inss * 0.225 - 675.49;
        } else if salary_after_inss > 4664.68 {
            imposto = salary_after_inss * 0.275 - 908.73;
        }

        // Redução adicional 2026
        let reducao = if salary_after_inss > 7350.0 {
            0.0
        } else {
            (978.62 - 0.133145 * salary_after_inss).max(0.0)
        };

        (imposto - reducao).max(0.0)
    };

    let pro_labore_percentage = move || -> ProLaborePercent {
        let inv = invoice
            .get()
            .replace("R$ ", "")
            .replace(".", "")
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0);
        let vale = vale
            .get()
            .replace("R$ ", "")
            .replace(".", "")
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0);
        let billing_monthly = (inv + vale) * 0.3;
        let pro_labore = if billing_monthly > minimum_salary {
            billing_monthly
        } else {
            minimum_salary
        };
        let inss_pro_labore = pro_labore * 0.11;
        let irrf_pro_labore = calc_irrf(pro_labore - inss_pro_labore);

        if !invoice.get().is_empty() {
            ProLaborePercent {
                pro_labore,
                inss_pro_labore,
                irrf_pro_labore,
            }
        } else {
            ProLaborePercent {
                pro_labore: 0.0,
                inss_pro_labore: 0.0,
                irrf_pro_labore: 0.0,
            }
        }
    };

    view! {
        <div class="uk-margin-small  uk-padding-small  uk-card uk-card-default uk-card-body">
            <span class="uk-card-title uk-text-secondary">
                "PJ - Simples nacional"
            </span>
            <hr class="uk-divider-icon" />
                <div class="uk-card">
                    <p class="uk-text-bolder">"Anexo III - 2024"</p>
                    <p class="uk-text-small uk-margin-remove">"(Fator R) Pro-labore > 28% do faturamento"</p>
                    <p class="uk-text-small uk-margin-remove">"Consideramos 30% de faturamento ou o salário mínimo"</p>
                    <hr class="uk-divider-small" />
                    <ul class="uk-list uk-list-hyphen">
                        <li class="uk-text-primary">
                            {move || format!("Faturamento: {}", format_currency(calc_anexo_III().billing) )}
                        </li>
                        <li class="uk-text-danger">
                            {move || format!("Simples Nacional a pagar: {}", format_currency(calc_anexo_III().tax_payable))}
                        </li>
                        <li class="uk-text-default">
                            {move || format!("Pro-labore: {}", format_currency(pro_labore_percentage().pro_labore))}
                            <p class="uk-text-small">"Apenas demonstrativo, não há ganho pois é provento próprio"</p>
                        </li>
                        <li class="uk-text-danger">
                            {move || format!("INSS sobre pro-labore: {}", format_currency(pro_labore_percentage().inss_pro_labore))}
                        </li>
                        <li class="uk-text-danger">
                            {move || format!("IRRF sobre pro-labore: {}", format_currency(pro_labore_percentage().irrf_pro_labore))}
                        </li>
                        <li class="uk-text-danger">
                            {move || format!("Despesas diversas: {}", format_currency(deduct_pj.get().replace("R$ ", "").replace(".", "").replace(",", ".").trim().parse().unwrap_or(0.0)))}
                        </li>


                    </ul>
                        <p class="uk-text-bolder">
                            {move || format!("Lucro líquido: {}", {
                                let net_profit = calc_anexo_III().billing - calc_anexo_III().tax_payable - pro_labore_percentage().inss_pro_labore - pro_labore_percentage().irrf_pro_labore - deduct_pj.get().replace("R$ ", "").replace(".", "").replace(",", ".").trim().parse().unwrap_or(0.0);
                                format_currency(net_profit)
                            })}
                        </p>
                </div>
                <hr class="uk-divider-icon" />
                <div class="uk-card">
                    <p class="uk-text-bolder">"Anexo V - 2024"</p>
                    <p class="uk-text-small uk-margin-remove">"(Fator R) Pro-labore < 28% do faturamento"</p>
                    <p class="uk-text-small uk-margin-remove">"Consideramos o salário minimo"</p>
                    <hr class="uk-divider-small" />
                    <ul class="uk-list uk-list-hyphen">
                        <li class="uk-text-primary">
                            {move || format!("Faturamento: {}", format_currency(calc_anexo_V().billing) )}
                        </li>
                        <li class="uk-text-danger">
                            {move || format!("Simples Nacional a pagar: {}", format_currency(calc_anexo_V().tax_payable))}
                        </li>
                        <li class="uk-text-default">
                            {move || format!("Pro-labore: {}", format_currency(pro_labore_minimum().minimum_salary))}
                            <p class="uk-text-small">"Apenas demonstrativo, não há ganho pois é provento próprio"</p>
                        </li>
                        <li class="uk-text-danger">
                            {move || format!("INSS sobre pro-labore: {}", format_currency(pro_labore_minimum().inss_pro_labore))}
                        </li>
                        <li class="uk-text-danger">
                            {move || format!("Despesas diversas: {}", format_currency(deduct_pj.get().replace("R$ ", "").replace(".", "").replace(",", ".").trim().parse().unwrap_or(0.0)))}
                        </li>
                    </ul>
                    <hr class="uk-divider-small" />
                    <p class="uk-text-bolder">
                        {move || format!("Lucro líquido: {}", {
                            let net_profit = calc_anexo_V().billing - calc_anexo_V().tax_payable - pro_labore_minimum().inss_pro_labore - deduct_pj.get().replace("R$ ", "").replace(".", "").replace(",", ".").trim().parse().unwrap_or(0.0);
                                format_currency(net_profit)
                        })}
                    </p>
                </div>
        </div>
    }
}
