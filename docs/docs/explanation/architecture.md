---
sidebar_position: 3
---

# Architecture

This document explains the architectural decisions behind Holt and why things
work the way they do.

## Why Behavior/Presentation Separation?

The behavior/presentation split comes from observing problems with traditional
component libraries:

**The coupling problem**: When styles and interactions are intertwined, changing
one risks breaking the other. A "simple" CSS change can inadvertently break
keyboard navigation or accessibility.

**The testing problem**: Testing a button's click handler shouldn't require
rendering visual styles, but coupled components make this difficult.

**The reuse problem**: You want a dropdown's keyboard navigation in a custom
design, but the library's dropdown looks nothing like your mockups. With coupled
components, you either accept the library's look or rewrite everything.

Separating behavior from presentation solves these:

- **Behaviors are stable contracts** - They handle state, keyboard input, and
  ARIA attributes. These don't change when you want a different visual style.
- **Presentations are replaceable** - Swap Tailwind classes, use a different
  design system, or create custom variants without touching behavior code.
- **Testing is focused** - Test behaviors with simple assertions about state.
  Test presentations with visual snapshots.

## Why Tailwind over CSS-in-Rust?

Rust has CSS-in-Rust solutions like Stylist and Leptos Style. Holt chose
Tailwind for several reasons:

**Ecosystem compatibility**: Tailwind has massive adoption. Designers use it,
tutorials assume it, and tooling supports it. Holt components work with your
existing Tailwind setup.

**No runtime cost**: `tailwind_fuse` resolves class conflicts at the call site.
There's no runtime CSS generation or injection.

**Type safety where it matters**: `tailwind_fuse` provides compile-time checks
for variant combinations and class merging, catching common errors without the
overhead of a full type system for CSS.

**Familiar patterns**: If you've used Shadcn UI or Radix in JavaScript, Holt's
styling approach will feel natural.

## Why Copy-Paste Components?

Holt follows the Shadcn model: components are meant to be copied into your
project and customized, not consumed as an opaque library.

**Your code, your control**: When you copy a component, you own it. No fighting
library opinions or waiting for upstream changes.

**Learning tool**: Reading and modifying component code teaches patterns you can
apply elsewhere.

**No version conflicts**: Copied code doesn't have version compatibility issues
with your other dependencies.

**Incremental adoption**: Copy the components you need. Skip the ones you don't.
Mix Holt components with your own.

The `holt-kit` crate provides ready-to-use components if you prefer a
traditional library approach, but the source is always available for
customization.

## Why a Storybook Framework?

Component development benefits from isolation. Holt Book provides:

**Visual testing**: See components in all their variants without building a full
application.

**Documentation**: Stories serve as living documentation showing how components
should be used.

**Iteration speed**: Hot reloading means changes appear instantly without full
rebuilds.

The story macro system makes adding stories low-friction - a few lines of code
creates a documented, interactive component example.
