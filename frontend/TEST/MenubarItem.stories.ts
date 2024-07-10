import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarItem from '../components/ui/menubar/MenubarItem.vue';

const meta = {
  title: 'MenubarItem',
  component: MenubarItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarItem>;

export default meta;
type Story = StoryObj<typeof MenubarItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};