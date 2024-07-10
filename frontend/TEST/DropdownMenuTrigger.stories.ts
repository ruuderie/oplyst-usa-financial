import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuTrigger from '../components/ui/dropdown-menu/DropdownMenuTrigger.vue';

const meta = {
  title: 'DropdownMenuTrigger',
  component: DropdownMenuTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuTrigger>;

export default meta;
type Story = StoryObj<typeof DropdownMenuTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};