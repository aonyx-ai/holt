// @component Table
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{
    Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};
use leptos::prelude::*;

#[variant]
fn invoices() -> AnyView {
    view! {
        <Table>
            <TableCaption>A list of your recent invoices.</TableCaption>
            <TableHeader>
                <TableRow>
                    <TableHead class="w-[100px]">Invoice</TableHead>
                    <TableHead>Status</TableHead>
                    <TableHead>Method</TableHead>
                    <TableHead class="text-right">Amount</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <TableRow>
                    <TableCell class="font-medium">INV001</TableCell>
                    <TableCell>Paid</TableCell>
                    <TableCell>Credit Card</TableCell>
                    <TableCell class="text-right">$250.00</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell class="font-medium">INV002</TableCell>
                    <TableCell>Pending</TableCell>
                    <TableCell>PayPal</TableCell>
                    <TableCell class="text-right">$150.00</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell class="font-medium">INV003</TableCell>
                    <TableCell>Unpaid</TableCell>
                    <TableCell>Bank Transfer</TableCell>
                    <TableCell class="text-right">$350.00</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell class="font-medium">INV004</TableCell>
                    <TableCell>Paid</TableCell>
                    <TableCell>Credit Card</TableCell>
                    <TableCell class="text-right">$450.00</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell class="font-medium">INV005</TableCell>
                    <TableCell>Paid</TableCell>
                    <TableCell>PayPal</TableCell>
                    <TableCell class="text-right">$550.00</TableCell>
                </TableRow>
            </TableBody>
            <TableFooter>
                <TableRow>
                    <TableCell attr:colspan="3">Total</TableCell>
                    <TableCell class="text-right">$1,750.00</TableCell>
                </TableRow>
            </TableFooter>
        </Table>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/table_source.rs"));

#[story(id = "table", name = "Table", extra_docs = TABLE_SOURCE)]
/// Styled HTML table for displaying tabular data
const TABLE_STORY: () = &[invoices];
