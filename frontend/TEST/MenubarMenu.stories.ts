import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarMenu from '../components/ui/menubar/MenubarMenu.vue';

const meta = {
  title: 'MenubarMenu',
  component: MenubarMenu,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarMenu>;

export default meta;
type Story = StoryObj<typeof MenubarMenu>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};