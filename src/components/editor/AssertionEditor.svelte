<script lang="ts">
  import { editorStore } from "../../lib/stores/editor.svelte";
  import { createEmptyAssertionRule, isAssertionComplete } from "../../lib/type-helpers";
  import type { AssertionOperator, AssertionRule } from "../../lib/types";

  const sourceOptions: { value: AssertionRule["source"]["type"]; label: string }[] = [
    { value: "statusCode", label: "Status Code" },
    { value: "responseTime", label: "Response Time (ms)" },
    { value: "header", label: "Header" },
    { value: "jsonPath", label: "JSONPath" },
    { value: "bodyContains", label: "Body Contains" },
  ];

  const operatorOptions: { value: AssertionOperator; label: string }[] = [
    { value: "equals", label: "==" },
    { value: "notEquals", label: "!=" },
    { value: "contains", label: "contains" },
    { value: "notContains", label: "not contains" },
    { value: "greaterThan", label: ">" },
    { value: "lessThan", label: "<" },
    { value: "exists", label: "exists" },
    { value: "notExists", label: "not exists" },
    { value: "typeIs", label: "type is" },
  ];

  function needsSourceValue(type: string): boolean {
    return type === "header" || type === "jsonPath" || type === "bodyContains";
  }

  function needsExpected(operator: AssertionOperator): boolean {
    return operator !== "exists" && operator !== "notExists";
  }

  function handleInput(index: number) {
    editorStore.markDirty();
    autoAddRow(index);
  }

  function autoAddRow(index: number) {
    const rules = editorStore.assertionRules;
    if (index === rules.length - 1) {
      const last = rules[index];
      if (last.expected !== null || !needsExpected(last.operator)) {
        editorStore.assertionRules = [...rules, createEmptyAssertionRule()];
      }
    }
  }

  function removeRule(index: number) {
    editorStore.assertionRules = editorStore.assertionRules.filter((_, i) => i !== index);
    editorStore.markDirty();
  }

  function toggleEnabled(index: number) {
    editorStore.assertionRules[index].isEnabled = !editorStore.assertionRules[index].isEnabled;
    editorStore.markDirty();
  }

  function updateSourceType(index: number, type: AssertionRule["source"]["type"]) {
    editorStore.assertionRules[index].source = { type, value: "" };
    editorStore.markDirty();
    autoAddRow(index);
  }

  function updateSourceValue(index: number, value: string) {
    editorStore.assertionRules[index].source = {
      ...editorStore.assertionRules[index].source,
      value,
    };
    editorStore.markDirty();
    autoAddRow(index);
  }

  function updateOperator(index: number, operator: AssertionOperator) {
    editorStore.assertionRules[index].operator = operator;
    if (!needsExpected(operator)) {
      editorStore.assertionRules[index].expected = null;
    }
    editorStore.markDirty();
    autoAddRow(index);
  }

  function updateExpected(index: number, value: string) {
    editorStore.assertionRules[index].expected = value;
    editorStore.markDirty();
    autoAddRow(index);
  }

  function isRulePopulated(rule: AssertionRule): boolean {
    return isAssertionComplete(rule);
  }
</script>

<div class="assertion-editor">
  <div class="header-row">
    <span class="col-check"></span>
    <span class="col-source">Source</span>
    <span class="col-op">Operator</span>
    <span class="col-expected">Expected</span>
    <span class="col-action"></span>
  </div>

  {#each editorStore.assertionRules as rule, index (rule.id)}
    <div class="row" class:disabled={!rule.isEnabled}>
      <span class="col-check">
        <input
          type="checkbox"
          checked={rule.isEnabled}
          onchange={() => toggleEnabled(index)}
        />
      </span>
      <span class="col-source">
        <select
          value={rule.source.type}
          onchange={(e) => updateSourceType(index, e.currentTarget.value as AssertionRule["source"]["type"])}
          disabled={!rule.isEnabled}
        >
          {#each sourceOptions as opt}
            <option value={opt.value}>{opt.label}</option>
          {/each}
        </select>
        {#if needsSourceValue(rule.source.type)}
          <input
            type="text"
            class="source-value"
            placeholder={rule.source.type === "header" ? "content-type" : rule.source.type === "jsonPath" ? "$.data.id" : "search text"}
            value={rule.source.value}
            oninput={(e) => updateSourceValue(index, e.currentTarget.value)}
            disabled={!rule.isEnabled}
          />
        {/if}
      </span>
      <span class="col-op">
        <select
          value={rule.operator}
          onchange={(e) => updateOperator(index, e.currentTarget.value as AssertionOperator)}
          disabled={!rule.isEnabled}
        >
          {#each operatorOptions as opt}
            <option value={opt.value}>{opt.label}</option>
          {/each}
        </select>
      </span>
      <span class="col-expected">
        {#if needsExpected(rule.operator)}
          <input
            type="text"
            placeholder="200"
            value={rule.expected ?? ""}
            oninput={(e) => updateExpected(index, e.currentTarget.value)}
            disabled={!rule.isEnabled}
          />
        {:else}
          <span class="no-value">—</span>
        {/if}
      </span>
      <span class="col-action">
        {#if isRulePopulated(rule)}
          <button class="delete-btn" onclick={() => removeRule(index)}>×</button>
        {/if}
      </span>
    </div>
  {/each}

  <p class="hint">
    Add assertions to validate responses. When assertions are defined, the collection runner uses them instead of the default 2xx status check.
  </p>
</div>

<style>
  .assertion-editor {
    display: flex;
    flex-direction: column;
    gap: 0;
  }
  .header-row {
    display: flex;
    align-items: center;
    gap: var(--sp-xs);
    padding: var(--sp-xs) 0;
    font-size: var(--fs-caption);
    font-weight: 600;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  .row {
    display: flex;
    align-items: flex-start;
    gap: var(--sp-xs);
    padding: 2px 0;
  }
  .row.disabled {
    opacity: 0.5;
  }
  .col-check {
    width: 30px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    padding-top: 6px;
  }
  .col-check input[type="checkbox"] {
    width: 14px;
    height: 14px;
    cursor: pointer;
  }
  .col-source {
    flex: 1.5;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .col-op {
    flex: 0.8;
  }
  .col-expected {
    flex: 1;
  }
  .col-action {
    width: 28px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    padding-top: 4px;
  }
  .row select,
  .row input[type="text"] {
    width: 100%;
    font-size: var(--fs-small);
    padding: var(--sp-xs) var(--sp-sm);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-sm);
    background: var(--bg-primary);
    color: var(--text-primary);
  }
  .row select:focus,
  .row input[type="text"]:focus {
    border-color: var(--color-info);
    outline: none;
  }
  .source-value {
    margin-top: 2px;
  }
  .no-value {
    display: block;
    padding: var(--sp-xs) var(--sp-sm);
    color: var(--text-tertiary);
    font-size: var(--fs-small);
  }
  .delete-btn {
    font-size: var(--fs-body);
    color: var(--text-tertiary);
    padding: 0;
    line-height: 1;
    background: transparent;
    border: none;
    cursor: pointer;
  }
  .delete-btn:hover {
    color: var(--color-error);
  }
  .hint {
    margin-top: var(--sp-md);
    font-size: var(--fs-caption);
    color: var(--text-tertiary);
    line-height: 1.4;
  }
</style>
