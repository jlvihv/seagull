<script lang="ts">
    import Alert from "./Alert.svelte";
    let username: string = "";
    let password: string = "";
    function login() {
        if (username === "" || password === "") {
            alert("用户名或密码不能为空");
            return;
        }
        console.log(username, password);
    }
</script>

<div
    class="relative flex min-h-screen flex-col justify-center overflow-hidden bg-gray-50 py-6 sm:py-12"
>
    <img
        src="/img/beams.jpg"
        alt=""
        class="absolute top-1/2 left-1/2 max-w-none -translate-x-1/2 -translate-y-1/2"
        width="1308"
    />
    <div
        class="absolute inset-0 bg-[url(/img/grid.svg)] bg-center [mask-image:linear-gradient(180deg,white,rgba(255,255,255,0))]"
    />
    <div
        class="relative bg-white px-6 pt-10 pb-8 shadow-xl ring-1 ring-gray-900/5 sm:mx-auto sm:max-w-lg sm:rounded-lg sm:px-10"
    >
        <div class="mx-auto max-w-md space-y-4">
            <h1>管理员登陆</h1>
            <div>
                <input
                    type="text"
                    placeholder="请输入用户名"
                    class="input input-bordered input-info w-full max-w-xs {username ===
                    ''
                        ? 'focus:input-error'
                        : ''}"
                    bind:value={username}
                />
            </div>
            <div>
                <input
                    type="password"
                    placeholder="请输入密码"
                    class="input input-bordered input-info w-full max-w-xs {password ===
                    ''
                        ? 'focus:input-error'
                        : ''}"
                    bind:value={password}
                />
            </div>
            <div>
                <button class="btn btn-block" on:click={login}> 登录 </button>
            </div>
        </div>
    </div>
</div>
