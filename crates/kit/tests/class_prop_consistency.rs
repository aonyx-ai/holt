/// Compilation test: every component's `class` prop must accept both
/// string literals (`&str`) and owned `String` values via `into`.
/// If this file compiles, the contract is satisfied.
use leptos::prelude::*;

// Behavior components
use holt_kit::behavior::{
    CheckboxIndicator, CheckboxRoot, CollapsibleContent, CollapsibleRoot, CollapsibleTrigger,
    SelectContent, SelectItem, SelectTrigger, SelectValue, SwitchRoot, SwitchThumb, ToggleRoot,
};

// Visual components
use holt_kit::visual::{
    Badge, Blockquote, Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator, Button, Card, CardContent, CardDescription, CardFooter, CardHeader,
    CardTitle, Checkbox, Collapsible, CollapsibleContent as CollapsibleContentVisual,
    CollapsibleTrigger as CollapsibleTriggerVisual, H1, H2, H3, H4, Input, Label, Large, Lead,
    Muted, P, Separator, Small, Switch, Textarea, Toggle,
};

/// Helper: constructs components with `class="literal"` and `class=String::from("dynamic")`.
/// We never actually render -- we only need the Leptos view macro to type-check.
#[component]
fn ClassPropTestHarness() -> impl IntoView {
    let open = RwSignal::new(false);
    let checked = RwSignal::new(false);
    let pressed = RwSignal::new(false);
    let _value = RwSignal::new(None::<String>);
    let text = RwSignal::new(String::new());

    // ---------- Behavior: string literal ----------
    let _b1 = view! {
        <CheckboxRoot class="lit" checked=checked>
            "x"
        </CheckboxRoot>
        <CheckboxIndicator class="lit">"x"</CheckboxIndicator>
        <CollapsibleRoot class="lit" open=open>
            "x"
        </CollapsibleRoot>
        <CollapsibleTrigger class="lit">"x"</CollapsibleTrigger>
        <CollapsibleContent class="lit">"x"</CollapsibleContent>
        <SelectTrigger class="lit">"x"</SelectTrigger>
        <SelectContent class="lit">"x"</SelectContent>
        <SelectItem value="a" class="lit">
            "x"
        </SelectItem>
        <SelectValue class="lit" />
        <SwitchRoot class="lit" checked=checked>
            "x"
        </SwitchRoot>
        <SwitchThumb class="lit" />
        <ToggleRoot class="lit" pressed=pressed>
            "x"
        </ToggleRoot>
    };

    // ---------- Behavior: owned String ----------
    let _b2 = view! {
        <CheckboxRoot class=String::from("dyn") checked=checked>
            "x"
        </CheckboxRoot>
        <CheckboxIndicator class=String::from("dyn")>"x"</CheckboxIndicator>
        <CollapsibleRoot class=String::from("dyn") open=open>
            "x"
        </CollapsibleRoot>
        <CollapsibleTrigger class=String::from("dyn")>"x"</CollapsibleTrigger>
        <CollapsibleContent class=String::from("dyn")>"x"</CollapsibleContent>
        <SelectTrigger class=String::from("dyn")>"x"</SelectTrigger>
        <SelectContent class=String::from("dyn")>"x"</SelectContent>
        <SelectItem value="a" class=String::from("dyn")>
            "x"
        </SelectItem>
        <SelectValue class=String::from("dyn") />
        <SwitchRoot class=String::from("dyn") checked=checked>
            "x"
        </SwitchRoot>
        <SwitchThumb class=String::from("dyn") />
        <ToggleRoot class=String::from("dyn") pressed=pressed>
            "x"
        </ToggleRoot>
    };

    // ---------- Visual: string literal ----------
    let _v1 = view! {
        <Badge class="lit">"x"</Badge>
        <Button class="lit">"x"</Button>
        <Card class="lit">"x"</Card>
        <CardHeader class="lit">"x"</CardHeader>
        <CardTitle class="lit">"x"</CardTitle>
        <CardDescription class="lit">"x"</CardDescription>
        <CardContent class="lit">"x"</CardContent>
        <CardFooter class="lit">"x"</CardFooter>
        <Input class="lit" value=text />
        <Textarea class="lit" value=text />
        <Label class="lit">"x"</Label>
        <Separator class="lit" />
        <Checkbox class="lit" checked=checked />
        <Switch class="lit" checked=checked />
        <Toggle class="lit">"x"</Toggle>
        <Collapsible class="lit" open=open>
            "x"
        </Collapsible>
        <CollapsibleTriggerVisual class="lit">"x"</CollapsibleTriggerVisual>
        <CollapsibleContentVisual class="lit">"x"</CollapsibleContentVisual>
        <H1 class="lit">"x"</H1>
        <H2 class="lit">"x"</H2>
        <H3 class="lit">"x"</H3>
        <H4 class="lit">"x"</H4>
        <P class="lit">"x"</P>
        <Blockquote class="lit">"x"</Blockquote>
        <Lead class="lit">"x"</Lead>
        <Large class="lit">"x"</Large>
        <Small class="lit">"x"</Small>
        <Muted class="lit">"x"</Muted>
        <Breadcrumb class="lit">"x"</Breadcrumb>
        <BreadcrumbList class="lit">"x"</BreadcrumbList>
        <BreadcrumbItem class="lit">"x"</BreadcrumbItem>
        <BreadcrumbLink class="lit">"x"</BreadcrumbLink>
        <BreadcrumbPage class="lit">"x"</BreadcrumbPage>
        <BreadcrumbSeparator class="lit" />
    };

    // ---------- Visual: owned String ----------
    let _v2 = view! {
        <Badge class=String::from("dyn")>"x"</Badge>
        <Button class=String::from("dyn")>"x"</Button>
        <Card class=String::from("dyn")>"x"</Card>
        <CardHeader class=String::from("dyn")>"x"</CardHeader>
        <CardTitle class=String::from("dyn")>"x"</CardTitle>
        <CardDescription class=String::from("dyn")>"x"</CardDescription>
        <CardContent class=String::from("dyn")>"x"</CardContent>
        <CardFooter class=String::from("dyn")>"x"</CardFooter>
        <Input class=String::from("dyn") value=text />
        <Textarea class=String::from("dyn") value=text />
        <Label class=String::from("dyn")>"x"</Label>
        <Separator class=String::from("dyn") />
        <Checkbox class=String::from("dyn") checked=checked />
        <Switch class=String::from("dyn") checked=checked />
        <Toggle class=String::from("dyn")>"x"</Toggle>
        <Collapsible class=String::from("dyn") open=open>
            "x"
        </Collapsible>
        <CollapsibleTriggerVisual class=String::from("dyn")>"x"</CollapsibleTriggerVisual>
        <CollapsibleContentVisual class=String::from("dyn")>"x"</CollapsibleContentVisual>
        <H1 class=String::from("dyn")>"x"</H1>
        <H2 class=String::from("dyn")>"x"</H2>
        <H3 class=String::from("dyn")>"x"</H3>
        <H4 class=String::from("dyn")>"x"</H4>
        <P class=String::from("dyn")>"x"</P>
        <Blockquote class=String::from("dyn")>"x"</Blockquote>
        <Lead class=String::from("dyn")>"x"</Lead>
        <Large class=String::from("dyn")>"x"</Large>
        <Small class=String::from("dyn")>"x"</Small>
        <Muted class=String::from("dyn")>"x"</Muted>
        <Breadcrumb class=String::from("dyn")>"x"</Breadcrumb>
        <BreadcrumbList class=String::from("dyn")>"x"</BreadcrumbList>
        <BreadcrumbItem class=String::from("dyn")>"x"</BreadcrumbItem>
        <BreadcrumbLink class=String::from("dyn")>"x"</BreadcrumbLink>
        <BreadcrumbPage class=String::from("dyn")>"x"</BreadcrumbPage>
        <BreadcrumbSeparator class=String::from("dyn") />
    };

    view! { <div>"Class prop consistency test harness"</div> }
}
