//! Living gallery for the `kesulu-ui` crate — renders every primitive. A standalone
//! CSR demo: run `trunk serve` in this crate — no server, auth, or host app.
use std::sync::Arc;

use leptos::prelude::*;

use kesulu_ui::*;

#[component]
fn Section(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="space-y-4">
            <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground border-b border-border pb-2">
                {title}
            </h2>
            <div class="flex flex-wrap items-start gap-8">{children()}</div>
        </section>
    }
}

#[component]
fn Demo(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <span class="text-[10px] uppercase tracking-wide text-muted-foreground/60">
                {label}
            </span>
            <div class="flex flex-wrap items-center gap-3">{children()}</div>
        </div>
    }
}

fn info_icon() -> AnyView {
    view! {
        <svg
            class="size-10"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <rect width="18" height="18" x="3" y="3" rx="2" />
            <path d="M3 9h18M9 21V9" />
        </svg>
    }
    .into_any()
}

#[component]
pub fn UiShowcasePage() -> impl IntoView {
    // Interactive state
    let dialog_open = RwSignal::new(false);
    let alert_open = RwSignal::new(false);
    let sheet_open = RwSignal::new(false);
    let popover_open = RwSignal::new(false);
    let select_val = RwSignal::new(String::new());
    let combo_val = RwSignal::new("NVDA".to_string());
    let native_val = RwSignal::new("med".to_string());
    let select_input_val = RwSignal::new("a".to_string());
    let collapsible_open = RwSignal::new(false);
    let tab = RwSignal::new("overview");
    let tab_underline = RwSignal::new("overview");
    let toggle_bold = RwSignal::new(false);
    let toggle_group_val = RwSignal::new("center".to_string());
    let switch_on = RwSignal::new(true);
    let checkbox_on = RwSignal::new(false);
    let radio_val = RwSignal::new("comfortable".to_string());
    let slider_val = RwSignal::new(40.0);
    let text_val = RwSignal::new(String::new());
    let num_val = RwSignal::new(10.0);
    let area_val = RwSignal::new(String::new());
    let error_visible = RwSignal::new(true);
    let progress_val = RwSignal::new(60.0);
    let toaster = Toaster::new();
    provide_context(toaster);
    let savebar_visible = RwSignal::new(false);
    let savebar_saving = RwSignal::new(false);

    view! {
        <div class="min-h-screen bg-background text-foreground">
            <ToastContainer />
            <SaveBar
                visible=savebar_visible
                saving=savebar_saving
                on_discard=Callback::new(move |()| savebar_visible.set(false))
                on_save=Callback::new(move |()| {
                    savebar_saving.set(true);
                    set_timeout(
                        move || {
                            savebar_saving.set(false);
                            savebar_visible.set(false);
                            toaster.success("Saved ✓");
                        },
                        std::time::Duration::from_millis(900),
                    );
                })
            />
            <header class="sticky top-0 z-30 border-b border-border bg-background/80 backdrop-blur px-8 py-4 flex items-center justify-between">
                <div>
                    <h1 class="text-lg font-bold tracking-tight">"ui — component gallery"</h1>
                    <p class="text-xs text-muted-foreground">
                        "shadcn/ui ported to Leptos · live SSR + hydrate"
                    </p>
                </div>
                <a
                    href="/"
                    class="text-xs text-muted-foreground hover:text-foreground underline-offset-4 hover:underline"
                >
                    "← back to app"
                </a>
            </header>

            <div class="mx-auto max-w-5xl px-8 py-10 space-y-12">

                <Section title="Buttons">
                    <Demo label="variants">
                        <Button>"Default"</Button>
                        <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                        <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
                        <Button variant=ButtonVariant::DestructiveMuted>"Destructive muted"</Button>
                        <Button variant=ButtonVariant::Outline>"Outline"</Button>
                        <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                        <Button variant=ButtonVariant::Link>"Link"</Button>
                    </Demo>
                    <Demo label="sizes / state">
                        <Button size=ButtonSize::Sm>"Small"</Button>
                        <Button size=ButtonSize::Lg>"Large"</Button>
                        <Button loading=true>"Loading"</Button>
                        <Button disabled=true>"Disabled"</Button>
                    </Demo>
                    <Demo label="button group">
                        <ButtonGroup>
                            <Button variant=ButtonVariant::Outline>"Year"</Button>
                            <Button variant=ButtonVariant::Outline>"Month"</Button>
                            <Button variant=ButtonVariant::Outline>"Day"</Button>
                        </ButtonGroup>
                    </Demo>
                </Section>

                <Section title="Icons">
                    <Demo label="glyphs (inherit text color via currentColor)">
                        <Icon name=IconName::ChevronDown />
                        <Icon name=IconName::ChevronLeft />
                        <Icon name=IconName::ChevronRight />
                        <Icon name=IconName::Check />
                        <Icon name=IconName::X />
                        <Icon name=IconName::Search />
                        <Icon name=IconName::Ellipsis />
                        <Icon name=IconName::CircleAlert />
                    </Demo>
                    <Demo label="size via class">
                        <Icon name=IconName::Search class="size-3.5" />
                        <Icon name=IconName::Search class="size-4" />
                        <Icon name=IconName::Search class="size-6" />
                        <Icon name=IconName::Search class="size-8" />
                    </Demo>
                    <Demo label="color via class">
                        <Icon name=IconName::CircleAlert class="size-5 text-accent" />
                        <Icon name=IconName::CircleAlert class="size-5 text-destructive" />
                        <Icon name=IconName::CircleAlert class="size-5 text-warning" />
                        <Icon name=IconName::CircleAlert class="size-5 text-muted-foreground" />
                    </Demo>
                </Section>

                <Section title="Badges & status">
                    <Demo label="badge variants">
                        <Badge text="Neutral" />
                        <Badge text="Success" variant=BadgeVariant::Success />
                        <Badge text="Danger" variant=BadgeVariant::Danger />
                        <Badge text="Warning" variant=BadgeVariant::Warning />
                        <Badge text="Info" variant=BadgeVariant::Info />
                    </Demo>
                    <Demo label="spinner / kbd">
                        <Spinner />
                        <KbdGroup>
                            <Kbd>"⌘"</Kbd>
                            <Kbd>"K"</Kbd>
                        </KbdGroup>
                    </Demo>
                    <Demo label="progress">
                        <div class="w-56">
                            <Progress value=Signal::from(progress_val) />
                        </div>
                    </Demo>
                </Section>

                <Section title="Form inputs">
                    <Demo label="text / number">
                        <div class="w-48">
                            <TextInput
                                value=text_val
                                on_input=Callback::new(move |v| text_val.set(v))
                                label="Symbol"
                                placeholder="AAPL"
                            />
                        </div>
                        <div class="w-40">
                            <NumberInput
                                value=Signal::from(num_val)
                                on_input=Callback::new(move |v| num_val.set(v))
                                label="Weight"
                                unit="%"
                            />
                        </div>
                    </Demo>
                    <Demo label="textarea">
                        <div class="w-72">
                            <TextArea
                                value=area_val
                                on_input=Callback::new(move |v| area_val.set(v))
                                label="Notes"
                                placeholder="Thesis…"
                            />
                        </div>
                    </Demo>
                    <Demo label="checkbox / switch">
                        <Checkbox
                            checked=checkbox_on
                            on_change=Callback::new(move |v| checkbox_on.set(v))
                            label="I agree"
                        />
                        <div class="flex items-center gap-2">
                            <Switch
                                checked=switch_on
                                on_change=Callback::new(move |v| switch_on.set(v))
                            />
                            <Label>"Live mode"</Label>
                        </div>
                    </Demo>
                    <Demo label="radio group">
                        <RadioGroup
                            value=radio_val
                            on_change=Callback::new(move |v| radio_val.set(v))
                        >
                            <RadioGroupItem value="default" label="Default" />
                            <RadioGroupItem value="comfortable" label="Comfortable" />
                            <RadioGroupItem value="compact" label="Compact" />
                        </RadioGroup>
                    </Demo>
                    <Demo label="slider">
                        <div class="w-56">
                            <Slider
                                value=slider_val
                                on_change=Callback::new(move |v| slider_val.set(v))
                                max=100.0
                            />
                        </div>
                    </Demo>
                </Section>

                <Section title="Selection">
                    <Demo label="select (popover)">
                        <Select
                            value=select_val
                            on_change=Callback::new(move |v| select_val.set(v))
                        >
                            <SelectTrigger>
                                <SelectValue placeholder="Pick a sector" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="tech">"Technology"</SelectItem>
                                <SelectItem value="health">"Healthcare"</SelectItem>
                                <SelectItem value="energy">"Energy"</SelectItem>
                            </SelectContent>
                        </Select>
                    </Demo>
                    <Demo label="combobox (searchable)">
                        <div class="w-64">
                            <Combobox
                                value=combo_val
                                on_change=Callback::new(move |v| combo_val.set(v))
                                options=vec![
                                    ComboboxOption::new("NVDA", "NVDA · NVIDIA"),
                                    ComboboxOption::new("AAPL", "AAPL · Apple"),
                                    ComboboxOption::new("MSFT", "MSFT · Microsoft"),
                                    ComboboxOption::new("AMD", "AMD · Advanced Micro Devices"),
                                    ComboboxOption::new("TSLA", "TSLA · Tesla"),
                                ]
                                placeholder="Search a symbol…"
                                search_placeholder="Type a ticker…"
                                empty_text="No symbol found."
                            />
                        </div>
                    </Demo>
                    <Demo label="native select">
                        <NativeSelect
                            value=native_val
                            on_change=Callback::new(move |v| native_val.set(v))
                        >
                            <NativeSelectOption value="low">"Low"</NativeSelectOption>
                            <NativeSelectOption value="med">"Medium"</NativeSelectOption>
                            <NativeSelectOption value="high">"High"</NativeSelectOption>
                        </NativeSelect>
                    </Demo>
                    <Demo label="select input (legacy)">
                        <SelectInput
                            value=select_input_val
                            on_change=Callback::new(move |v| select_input_val.set(v))
                            options=vec![
                                ("a".to_string(), "Alpha".to_string()),
                                ("b".to_string(), "Beta".to_string()),
                            ]
                        />
                    </Demo>
                    <Demo label="toggle / toggle group">
                        <Toggle
                            pressed=toggle_bold
                            on_change=Callback::new(move |v| toggle_bold.set(v))
                        >
                            "B"
                        </Toggle>
                        <ToggleGroup
                            value=toggle_group_val
                            on_change=Callback::new(move |v| toggle_group_val.set(v))
                        >
                            <ToggleGroupItem value="left">"L"</ToggleGroupItem>
                            <ToggleGroupItem value="center">"C"</ToggleGroupItem>
                            <ToggleGroupItem value="right">"R"</ToggleGroupItem>
                        </ToggleGroup>
                    </Demo>
                </Section>

                <Section title="Overlays (animated enter / exit)">
                    <Demo label="dialog">
                        <Button on:click=move |_| dialog_open.set(true)>"Open dialog"</Button>
                        <Dialog open=dialog_open>
                            <DialogHeader>
                                <DialogTitle>"Rebalance now?"</DialogTitle>
                                <DialogDescription>
                                    "This submits the current target weights to the broker."
                                </DialogDescription>
                            </DialogHeader>
                            <DialogFooter>
                                <Button
                                    variant=ButtonVariant::Outline
                                    on:click=move |_| dialog_open.set(false)
                                >
                                    "Cancel"
                                </Button>
                                <Button on:click=move |_| dialog_open.set(false)>"Confirm"</Button>
                            </DialogFooter>
                        </Dialog>
                    </Demo>
                    <Demo label="alert dialog">
                        <Button
                            variant=ButtonVariant::Destructive
                            on:click=move |_| alert_open.set(true)
                        >
                            "Delete"
                        </Button>
                        <AlertDialog
                            open=alert_open
                            title="Delete strategy?"
                            description="This action cannot be undone."
                            confirm_label="Delete"
                            variant=ButtonVariant::Destructive
                            on_confirm=Arc::new(|| ())
                        />
                    </Demo>
                    <Demo label="sheet">
                        <Button
                            variant=ButtonVariant::Outline
                            on:click=move |_| sheet_open.set(true)
                        >
                            "Open sheet"
                        </Button>
                        <Sheet open=sheet_open>
                            <SheetContent open=sheet_open side=SheetSide::Right>
                                <SheetHeader>
                                    <SheetTitle>"Filters"</SheetTitle>
                                    <SheetDescription>"Refine the universe."</SheetDescription>
                                </SheetHeader>
                                <div class="p-4 text-sm text-muted-foreground">
                                    "Body content…"
                                </div>
                            </SheetContent>
                        </Sheet>
                    </Demo>
                    <Demo label="popover">
                        <Popover open=popover_open>
                            <PopoverTrigger>
                                <Button variant=ButtonVariant::Outline>"Open popover"</Button>
                            </PopoverTrigger>
                            <PopoverContent>
                                <PopoverHeader>
                                    <PopoverTitle>"Dimensions"</PopoverTitle>
                                    <PopoverDescription>
                                        "Set the size constraints."
                                    </PopoverDescription>
                                </PopoverHeader>
                            </PopoverContent>
                        </Popover>
                    </Demo>
                    <Demo label="dropdown menu">
                        <DropdownMenu trigger=view! {
                            <Button variant=ButtonVariant::Outline>"Actions ▾"</Button>
                        }
                            .into_any()>
                            <DropdownMenuItem>"Edit"</DropdownMenuItem>
                            <DropdownMenuItem>"Duplicate"</DropdownMenuItem>
                            <DropdownMenuSeparator />
                            <DropdownMenuItem>"Delete"</DropdownMenuItem>
                        </DropdownMenu>
                    </Demo>
                    <Demo label="tooltip / hover card">
                        <Tooltip content="Net asset value">
                            <Button variant=ButtonVariant::Ghost>"NAV"</Button>
                        </Tooltip>
                        <HoverCard>
                            <HoverCardTrigger>
                                <Button variant=ButtonVariant::Link>"@kesulu"</Button>
                            </HoverCardTrigger>
                            <HoverCardContent>
                                <p class="text-sm">"Systematic US-equity momentum."</p>
                            </HoverCardContent>
                        </HoverCard>
                    </Demo>
                </Section>

                <Section title="Disclosure">
                    <Demo label="tabs">
                        <div class="w-80">
                            <Tabs value=tab>
                                <TabsList>
                                    <TabsTrigger value="overview">"Overview"</TabsTrigger>
                                    <TabsTrigger value="holdings">"Holdings"</TabsTrigger>
                                </TabsList>
                                <TabsContent value="overview">
                                    <p class="text-sm text-muted-foreground">"Overview panel."</p>
                                </TabsContent>
                                <TabsContent value="holdings">
                                    <p class="text-sm text-muted-foreground">"Holdings panel."</p>
                                </TabsContent>
                            </Tabs>
                        </div>
                    </Demo>
                    <Demo label="tabs · underline">
                        <div class="w-80">
                            <Tabs value=tab_underline variant=TabsVariant::Underline>
                                <TabsList>
                                    <TabsTrigger value="overview">"Overview"</TabsTrigger>
                                    <TabsTrigger value="holdings">"Holdings"</TabsTrigger>
                                </TabsList>
                                <TabsContent value="overview">
                                    <p class="text-sm text-muted-foreground">"Overview panel."</p>
                                </TabsContent>
                                <TabsContent value="holdings">
                                    <p class="text-sm text-muted-foreground">"Holdings panel."</p>
                                </TabsContent>
                            </Tabs>
                        </div>
                    </Demo>
                    <Demo label="accordion">
                        <div class="w-80">
                            <Accordion default_value="a">
                                <AccordionItem value="a">
                                    <AccordionTrigger>"What is momentum?"</AccordionTrigger>
                                    <AccordionContent>
                                        "Buying recent relative winners."
                                    </AccordionContent>
                                </AccordionItem>
                                <AccordionItem value="b">
                                    <AccordionTrigger>"Rebalance cadence?"</AccordionTrigger>
                                    <AccordionContent>"Weekly."</AccordionContent>
                                </AccordionItem>
                            </Accordion>
                        </div>
                    </Demo>
                    <Demo label="collapsible">
                        <div class="w-80">
                            <Collapsible open=collapsible_open>
                                <CollapsibleTrigger class="w-full">
                                    <Button variant=ButtonVariant::Outline class="w-full">
                                        "Toggle details"
                                    </Button>
                                </CollapsibleTrigger>
                                <CollapsibleContent class="pt-2 text-sm text-muted-foreground">
                                    "Hidden details revealed."
                                </CollapsibleContent>
                            </Collapsible>
                        </div>
                    </Demo>
                </Section>

                <Section title="Data display">
                    <Demo label="card">
                        <Card class="w-72">
                            <CardHeader>
                                <CardTitle>"Sharpe ratio"</CardTitle>
                                <CardDescription>"Trailing 12 months"</CardDescription>
                            </CardHeader>
                            <CardContent>
                                <p class="text-3xl font-bold">"1.84"</p>
                            </CardContent>
                            <CardFooter>
                                <Badge text="Healthy" variant=BadgeVariant::Success />
                            </CardFooter>
                        </Card>
                    </Demo>
                    <Demo label="kpi · joined strip (Lg + sub)">
                        <div class="grid grid-cols-3 gap-px bg-border rounded-xl overflow-hidden border border-border w-[440px]">
                            <KPICard
                                label="Total Equity"
                                value="$100,000.00".into_any()
                                sub_label=view! { <span class="text-accent">"Today +0.4%"</span> }
                                    .into_any()
                            />
                            <KPICard
                                label="Cash"
                                value="$5,230.35".into_any()
                                sub_label=view! {
                                    <span class="text-muted-foreground">"Buffer 5.2%"</span>
                                }
                                    .into_any()
                            />
                            <KPICard
                                label="Return vs SPX"
                                value="+24.0%".into_any()
                                value_class="text-accent"
                                sub_label=view! {
                                    <span class="text-muted-foreground">"\u{3b1} +8.0%"</span>
                                }
                                    .into_any()
                            />
                        </div>
                    </Demo>
                    <Demo label="kpi · Md metric grid (no sub)">
                        <div class="grid grid-cols-4 gap-px bg-border rounded-xl overflow-hidden border border-border w-[440px]">
                            <KPICard size=KpiSize::Md label="Sharpe" value="1.84".into_any() />
                            <KPICard size=KpiSize::Md label="Sortino" value="2.31".into_any() />
                            <KPICard
                                size=KpiSize::Md
                                label="Max DD"
                                value="-12.4%".into_any()
                                value_class="text-destructive"
                            />
                            <KPICard size=KpiSize::Md label="Win Rate" value="58%".into_any() />
                        </div>
                    </Demo>
                    <Demo label="kpi · bare Sm stats (inside a card)">
                        <Card class="p-4 w-[360px]">
                            <div class="grid grid-cols-3 gap-5">
                                <KPICard
                                    bare=true
                                    size=KpiSize::Sm
                                    label="Momentum"
                                    value="1.823".into_any()
                                />
                                <KPICard
                                    bare=true
                                    size=KpiSize::Sm
                                    label="Rank"
                                    value="#1".into_any()
                                />
                                <KPICard
                                    bare=true
                                    size=KpiSize::Sm
                                    label="Vol 20d"
                                    value="24.1%".into_any()
                                />
                            </div>
                        </Card>
                    </Demo>
                    <Demo label="avatar">
                        <AvatarGroup>
                            <Avatar>
                                <AvatarFallback>"KS"</AvatarFallback>
                            </Avatar>
                            <Avatar>
                                <AvatarFallback>"AI"</AvatarFallback>
                            </Avatar>
                            <AvatarGroupCount>"+3"</AvatarGroupCount>
                        </AvatarGroup>
                    </Demo>
                    <Demo label="table">
                        <Table class="w-96">
                            <TableHeader>
                                <TableRow>
                                    <TableHead>"Symbol"</TableHead>
                                    <TableHead>"Weight"</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                <TableRow>
                                    <TableCell>"AAPL"</TableCell>
                                    <TableCell>"12.4%"</TableCell>
                                </TableRow>
                                <TableRow>
                                    <TableCell>"MSFT"</TableCell>
                                    <TableCell>"9.1%"</TableCell>
                                </TableRow>
                            </TableBody>
                        </Table>
                    </Demo>
                    <Demo label="skeleton">
                        <div class="w-64">
                            <SkeletonBlock rows=3 />
                        </div>
                    </Demo>
                </Section>

                <Section title="Feedback">
                    <Demo label="alert">
                        <div class="w-96 space-y-3">
                            <Alert>
                                <AlertTitle>"Heads up"</AlertTitle>
                                <AlertDescription>"Markets close early today."</AlertDescription>
                            </Alert>
                            <Alert variant=AlertVariant::Warning>
                                <AlertTitle>"Survivorship caveat"</AlertTitle>
                                <AlertDescription>
                                    "Latest universe applied to history — returns overstated."
                                </AlertDescription>
                            </Alert>
                            <Alert variant=AlertVariant::Destructive>
                                <AlertTitle>"Order rejected"</AlertTitle>
                                <AlertDescription>"Insufficient buying power."</AlertDescription>
                            </Alert>
                        </div>
                    </Demo>
                    <Demo label="error banner">
                        <div class="w-96">
                            <ErrorBanner
                                visible=error_visible
                                message="Failed to load quotes."
                                on_retry=Callback::new(move |()| error_visible.set(false))
                            />
                        </div>
                    </Demo>
                    <Demo label="empty state">
                        <div class="w-96 border border-border rounded-lg">
                            <EmptyState
                                icon=info_icon()
                                title="No positions"
                                description="Run a sim cycle to populate holdings."
                            />
                        </div>
                    </Demo>
                    <Demo label="toast (top-right)">
                        <Button
                            size=ButtonSize::Sm
                            on:click=move |_| toaster.success("Cycle complete")
                        >
                            "Success"
                        </Button>
                        <Button
                            size=ButtonSize::Sm
                            variant=ButtonVariant::Destructive
                            on:click=move |_| toaster.error("Order rejected")
                        >
                            "Error"
                        </Button>
                        <Button
                            size=ButtonSize::Sm
                            variant=ButtonVariant::Outline
                            on:click=move |_| toaster.info("Markets open")
                        >
                            "Info"
                        </Button>
                    </Demo>
                    <Demo label="save bar (sticky, presence + loading + toast)">
                        <Button size=ButtonSize::Sm on:click=move |_| savebar_visible.set(true)>
                            "Show save bar"
                        </Button>
                    </Demo>
                </Section>

                <Section title="Navigation & layout">
                    <Demo label="breadcrumb">
                        <Breadcrumb>
                            <BreadcrumbList>
                                <BreadcrumbItem>
                                    <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator />
                                <BreadcrumbItem>
                                    <BreadcrumbLink href="/universe">"Universe"</BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator />
                                <BreadcrumbItem>
                                    <BreadcrumbPage>"AAPL"</BreadcrumbPage>
                                </BreadcrumbItem>
                            </BreadcrumbList>
                        </Breadcrumb>
                    </Demo>
                    <Demo label="pagination">
                        <Pagination>
                            <PaginationContent>
                                <PaginationItem>
                                    <PaginationPrevious href="#" />
                                </PaginationItem>
                                <PaginationItem>
                                    <PaginationLink href="#">"1"</PaginationLink>
                                </PaginationItem>
                                <PaginationItem>
                                    <PaginationLink href="#" is_active=true>
                                        "2"
                                    </PaginationLink>
                                </PaginationItem>
                                <PaginationItem>
                                    <PaginationEllipsis />
                                </PaginationItem>
                                <PaginationItem>
                                    <PaginationNext href="#" />
                                </PaginationItem>
                            </PaginationContent>
                        </Pagination>
                    </Demo>
                    <Demo label="section header">
                        <div class="w-96">
                            <SectionHeader title="Holdings" />
                        </div>
                    </Demo>
                    <Demo label="separator / aspect ratio">
                        <div class="flex items-center gap-3 text-sm">
                            "Live" <Separator vertical=true class="h-4" /> "Sim"
                        </div>
                        <div class="w-48">
                            <AspectRatio ratio=1.777>
                                <div class="h-full w-full rounded-md bg-muted flex items-center justify-center text-xs text-muted-foreground">
                                    "16 : 9"
                                </div>
                            </AspectRatio>
                        </div>
                    </Demo>
                </Section>

            </div>
        </div>
    }
}
