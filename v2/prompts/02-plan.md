# Stage 2 kickoff — PLAN (SPEC.md → FEATURES.md)

Start a session **in plan mode**, then:

---

Read SPEC.md. Produce FEATURES.md per the template in
v2/FEATURES.md.template:

- Decompose every behavior in the spec into falsifiable claims — each with
  a concrete Verify command (a test file path or script). Err on the side
  of MORE, smaller features; 50-200 items for a real product is normal.
  Include the unglamorous ones: error states, empty states, auth edges,
  migrations, observability.
- Group into vertical slices, each shippable end-to-end and sized to one
  PR. Order slices by dependency; flag which can run in parallel.
- Mark everything `failing`.
- Copy SPEC.md's non-goals into the bottom section verbatim.
- Flag any feature where the spec is ambiguous as a question for me now,
  not during the build.

Present the slice plan for my approval before writing the file.
