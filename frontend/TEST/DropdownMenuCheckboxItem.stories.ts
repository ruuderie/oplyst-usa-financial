import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuCheckboxItem from '../components/ui/dropdown-menu/DropdownMenuCheckboxItem.vue';

const meta = {
  title: 'DropdownMenuCheckboxItem',
  component: DropdownMenuCheckboxItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuCheckboxItem>;

export default meta;
type Story = StoryObj<typeof DropdownMenuCheckboxItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};