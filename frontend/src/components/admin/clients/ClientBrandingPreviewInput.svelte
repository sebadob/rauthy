<script>
	import { slide } from "svelte/transition";
	import IconEye from "$lib/icons/IconEye.svelte";
	import IconEyeSlash from "$lib/icons/IconEyeSlash.svelte";

	export let colors = {};
	export let error = '';
	export let name = '';
	export let password = false;
	export let value = '';
	export let width = '250px';

	let type = password ? 'new-password' : 'text';
	let isHover = false;
	let isFocus = false;

	function toggle() {
		if (type === 'password') {
			type = 'text';
		} else {
			type = 'password';
		}
	}

</script>

<div
    class="container"
    style:width="calc({width} + 12px)"
>
  <div class="label">
    <div
        class="labelInner font-label noselect"
        style:background={colors.bg}
    >
      <label for={name}>
        <slot></slot>
      </label>
    </div>
  </div>

  <input
      {type}
      on:mouseenter={() => isHover = true}
      on:mouseleave={() => isHover = false}
      on:focus={() => isFocus = true}
      on:blur={() => isFocus = false}
      autocomplete="off"
      style:width={width}
      style:background={isHover || isFocus ? colors.bg : colors.bg}
      style:border={isFocus ? `1px solid ${colors.acnt}` : `1px solid ${colors.glow}` }
      style:color={colors.text}
      style:box-shadow="1px 1px 2px {colors.gmid}"
      {name}
      {value}
      {...$$restProps}
  />

  {#if password}
    <div class="rel">
      <div
              role="none"
              class="btn show"
              on:click={toggle}
              on:keypress={toggle}
      >
        {#if type === 'password'}
          <IconEyeSlash width={22} color={colors.err} />
        {:else}
          <IconEye width={22} color={colors.ok} />
        {/if}
      </div>
    </div>
  {/if}

  {#if error}
    <div
        class="err font-label"
        style:width={width}
        style:color={colors.err}
        transition:slide|global={{ duration: 250 }}
    >
      {error}
    </div>
  {/if}
</div>

<style>
    .btn {
        position: absolute;
        top: -32px;
        right: 10px;
        margin-left: 100px;
        opacity: 0.85;
        cursor: pointer;
    }

    .container {
        display: flex;
        flex-direction: column;
        margin-bottom: 3px;
    }

    input {
        margin: 12px 5px 5px 5px;
        padding: 5px 10px;
        border-radius: 7px;
        font-size: 1.05rem;
        outline: none;
    }

    .label {
        position: relative;
    }

    .err {
        margin-top: -6px;
        margin-left: 5px;
        padding: 0 10px;
        font-size: .85rem;
    }

    .labelInner {
        position: absolute;
        top: 0;
        left: 11px;
        border-radius: 5px;
        padding: 0 5px;
        font-size: .8rem;
    }

    .rel {
        position: relative;
    }

    .show {
        opacity: 0.85;
    }
</style>
