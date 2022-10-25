import { Head } from "$fresh/runtime.ts";
import { HandlerContext, Handlers, PageProps } from "$fresh/server.ts"
import Kiosk from "../islands/kiosk.tsx";
import { Category, Menu } from "../utils/types.tsx";

interface KioskProps {
  categories: Array<Category>;
  menus: Array<Menu>;
}

export const handler: Handlers = {
  async GET(_req: Request, ctx: HandlerContext): Promise<Response> {
    console.log(Deno.env.get('BASE_URL'))
    const res = await fetch(`${Deno.env.get('BASE_URL')}/order`);
    const menus = await res.json();
    return await ctx.render({
      categories: [{ key: 'ALL', name: '전체'}, { key: 'COFFEE', name: '커피' }, { key: 'TEA', name: '차' }],
      menus,
    });
  }
}

export default function Home({ data }: PageProps<KioskProps>) {
  return (
    <>
      <Head>
        <title>kiosk</title>
      </Head>
      <div class="container mx-auto grid grid-cols-1 gap-2 md:grid-cols-2">
        <div class="mx-auto w-full h-screen relative">
          <Kiosk categories={data.categories} menus={data.menus} />
        </div>
        <div class="mx-auto h-screen">
          this area is order state
        </div>
      </div>
    </>
  );
}
