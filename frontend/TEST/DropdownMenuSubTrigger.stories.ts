import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuSubTrigger from '../components/ui/dropdown-menu/DropdownMenuSubTrigger.vue';

const meta = {
  title: 'DropdownMenuSubTrigger',
  component: DropdownMenuSubTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuSubTrigger>;

export default meta;
type Story = StoryObj<typeof DropdownMenuSubTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};