import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarShortcut from '../components/ui/menubar/MenubarShortcut.vue';

const meta = {
  title: 'MenubarShortcut',
  component: MenubarShortcut,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarShortcut>;

export default meta;
type Story = StoryObj<typeof MenubarShortcut>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};