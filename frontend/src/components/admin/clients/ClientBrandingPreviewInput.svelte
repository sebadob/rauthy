<script>
	import { slide } from "svelte/transition";
	import IconEye from "$lib/icons/IconEye.svelte";
	import IconEyeSlash from "$lib/icons/IconEyeSlash.svelte";

  /**
   * @typedef {Object} Props
   * @property {any} [colors]
   * @property {string} [error]
   * @property {string} [name]
   * @property {boolean} [password]
   * @property {string} [value]
   * @property {string} [width]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props & { [key: string]: any }} */
  let {
    colors = {},
    error = '',
    name = '',
    password = false,
    value = '',
    width = '250px',
    children,
    ...rest
  } = $props();

	let type = $state(password ? 'new-password' : 'text');
	let isHover = $state(false);
	let isFocus = $state(false);

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
        {@render children?.()}
      </label>
    </div>
  </div>

  <input
      {type}
      onmouseenter={() => isHover = true}
      onmouseleave={() => isHover = false}
      onfocus={() => isFocus = true}
      onblur={() => isFocus = false}
      autocomplete="off"
      style:width={width}
      style:background={isHover || isFocus ? colors.bg : colors.bg}
      style:border={isFocus ? `1px solid ${colors.acnt}` : `1px solid ${colors.glow}` }
      style:color={colors.text}
      style:box-shadow="1px 1px 2px {colors.gmid}"
      {name}
      {value}
      {...rest}
  />

  {#if password}
    <div class="rel">
      <div
              role="none"
              class="btn show"
              onclick={toggle}
              onkeypress={toggle}
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
