import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuSeparator from '../components/ui/dropdown-menu/DropdownMenuSeparator.vue';

const meta = {
  title: 'DropdownMenuSeparator',
  component: DropdownMenuSeparator,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuSeparator>;

export default meta;
type Story = StoryObj<typeof DropdownMenuSeparator>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};