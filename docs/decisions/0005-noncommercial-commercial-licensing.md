# ADR-0005: Noncommercial community use and separate commercial licenses

**Status:** Adopted on the owner's explicit instruction; legal review remains recommended.

## Decision

Allow personal noncommercial use, research, teaching, modification, private forks,
redistribution with notices, and community participation without a fee. Reserve
commercial exploitation for a separate written agreement with Christian Klose;
the current license price is EUR 20,000. Do not invent a subscription period,
perpetual company-wide grant, OEM entitlement, or automatic license on payment.

Use the explicitly project-specific `NaOnDB Noncommercial License 1.0` with a
`LicenseRef-NaOnDB-Noncommercial-1.0` identifier, not a misleading standard-license
identifier. Cargo points to the actual `LICENSE` through `license-file`.
Do not claim OSI open-source status: the commercial restriction is intentional.

## Why not unmodified PolyForm Noncommercial 1.0.0?

Its `Noncommercial Organizations` clause broadly permits use by specified
institutions regardless of funding obligations. The owner's boundary is based
on the activity: research and community participation are welcome, but an
institutional label alone must not exempt commercial exploitation. Our LICENSE
states that narrower boundary explicitly, while allowing salaries, grants,
sponsorship, and academic publication that do not themselves turn the activity
into commercial product development or operations. This is custom legal drafting,
not a claim of a professionally vetted PolyForm variant.

## Contribution rights

Contributors retain ownership. A separate, explicitly accepted nonexclusive
agreement permits Christian Klose to include the identified contributions in
community releases and paid commercial licenses for NaOnDB and derivatives.
There is no promised contractual revenue share, and no implicit assent by
existing contributors. Third-party rights and mandatory statutory rights remain
unchanged. Acceptance is reviewed manually before merge, not automatically enforced.

## Consequences

Some organizations will decline custom or noncommercial licenses. That trade-off
follows the owner's commercial restriction. Review these texts with qualified
counsel before substantial external contributions or the first commercial deal.
The first commercial agreement still needs its actual scope, term, tax treatment,
licensee, and payment conditions. No money is collected and no package is published
by this decision. Pricing changes do not retrospectively alter existing grants.

## Primary references consulted

- PolyForm Noncommercial 1.0.0: https://polyformproject.org/licenses/noncommercial/1.0.0
- OSI Open Source Definition, section 6: https://opensource.org/osd
- Cargo license-file metadata: https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields
