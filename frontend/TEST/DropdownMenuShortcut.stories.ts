import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuShortcut from '../components/ui/dropdown-menu/DropdownMenuShortcut.vue';

const meta = {
  title: 'DropdownMenuShortcut',
  component: DropdownMenuShortcut,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuShortcut>;

export default meta;
type Story = StoryObj<typeof DropdownMenuShortcut>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};