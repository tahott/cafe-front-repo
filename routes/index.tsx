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
    return await ctx.render({
      categories: [{ key: 'all', name: '전체'}, { key: 'coffee', name: '커피' }, { key: 'tea', name: '차' }],
      menus: [
        { key: 'coffee', name: '아메리카노', price: 3000},
        { key: 'coffee', name: '라떼', price: 3500 },
        { key: 'tea', name: '귤차', price: 3000 },
      ]
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
